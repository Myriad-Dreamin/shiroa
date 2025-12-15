use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use blake3::Hasher;

use crate::{
    error::prelude::*,
    utils::{create_dirs, make_absolute_from},
};

/// Static asset to be copied to output directory
#[derive(Debug, Clone, PartialEq)]
pub struct AssetInput {
    pub src: AssetSource,
    /// Destination path (made absolute later)
    pub dest: String,
    /// Asset type hint (css, js, font, image, etc.)
    pub asset_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetSource {
    /// Source path (absolute)
    Path(PathBuf),
    /// Direct content bytes
    Bytes(Vec<u8>),
}

/// Result about what happened for an asset submission.
#[derive(Debug)]
pub struct ProcessedAsset {
    pub dest: PathBuf,
    pub written: bool, // true if the file was written/overwritten on disk
}

/// In-memory record for a destination.
#[derive(Clone, Debug)]
struct DestRecord {
    hash: String,
    owner: Option<String>, // page id that created/owns it in this run (None = unknown / preexisting)
                           // dest: PathBuf,
}
// TODO: the dest can change at runtime, so we may need to track more info per dest.

/// Thread-safe manager for parallel page compilers.
#[derive(Clone, Default)]
pub struct AssetManager(Arc<Mutex<AssetManagerInner>>);

/// Shared inner state protected by Mutex.
#[derive(Default)]
struct AssetManagerInner {
    dest_map: HashMap<PathBuf, DestRecord>,
    page_dependencies: HashMap<String, HashSet<PathBuf>>, // page_id -> list of dest paths. not used yet.
}

impl AssetManager {
    pub fn new() -> Self {
        Default::default()
    }

    /// Submit a page's assets. Can be called concurrently from many threads.
    /// - For each asset: compute its hash, then under lock apply the rules:
    ///   * same-page update -> overwrite allowed
    ///   * different-page different-hash -> Conflict error
    ///   * identical-hash anywhere -> reuse/skip write
    ///
    /// Returns ProcessedAsset list (same order as inputs).
    pub fn submit_assets(
        &self,
        page_id: &str,
        inputs: &[AssetInput],
        dist_dir: &Path,
    ) -> Result<()> {
        // 1) compute hashes before locking
        let mut items: Vec<(AssetInput, String)> = Vec::with_capacity(inputs.len());
        for input in inputs.iter() {
            let hash = compute_hash_of_source(&input.src).context("compute_hash_of_file")?;
            items.push((input.clone(), hash));
        }
        log::info!("Submitting {} assets for page {}", items.len(), page_id);

        // 2) lock and apply rules & write under lock (keeping operations as short as practical)
        // Alternative (more advanced): reserve under lock, release, perform IO, then re-lock to commit/rollback.
        // For simplicity and correctness we keep write under the lock here.
        let mut inner = self.0.lock().unwrap();
        let inner = &mut *inner;

        create_dirs(dist_dir)?;

        let dest_map = &mut inner.dest_map;
        let mut dependencies = HashSet::new();

        for (input, hash) in items.into_iter() {
            let dest_path = make_absolute_from(Path::new(&input.dest), || dist_dir.to_path_buf());

            // validate that dest is within dist_dir
            if !dest_path.starts_with(dist_dir) {
                log::warn!(
                    "Asset destination {dest_path:?} is outside of dist dir {dist_dir:?}, skipping"
                );
                continue;
            }
            // check that source exists (for Path variant)
            if let AssetSource::Path(p) = &input.src {
                if !p.exists() {
                    log::warn!("Asset source path {p:?} does not exist, skipping");
                    continue;
                }
            }

            dependencies.insert(dest_path.clone());

            // check in-memory record
            if let Some(rec) = dest_map.get(&dest_path) {
                if rec.hash == hash {
                    log::debug!(
                        "Asset at {dest_path:?} already exists with same hash {hash}, skipping write"
                    );
                    // same content => ok, no write
                    continue;
                }

                // different content
                if rec.owner.as_deref() != Some(page_id) {
                    log::warn!(
                        "Asset conflict at {dest_path:?}: existing hash {}, new hash {}",
                        rec.hash,
                        hash
                    );
                    continue;
                }
                log::info!(
                    "Asset at {dest_path:?} being overwritten by same page {page_id}, hash {} -> {}",
                    rec.hash,
                    hash
                );
                // same page owns it previously -> allow overwrite
                write_atomic_from_source(&input.src, &dest_path).context("submit create dirs")?;
                // update record
                dest_map.insert(
                    dest_path.clone(),
                    DestRecord {
                        hash: hash.clone(),
                        owner: Some(page_id.to_string()),
                    },
                );
                continue;
            }

            // no in-memory record: check on-disk
            if dest_path.exists() {
                let disk_hash = compute_hash_of_file(&dest_path).context("compute_hash_of_file")?;
                if disk_hash != hash {
                    // on-disk content differs => conflict
                    log::warn!(
                        "Asset conflict at {dest_path:?}: existing on-disk hash {disk_hash}, new hash {hash}",
                    );
                }
                // file on disk matches -> record with owner = None or optionally claim owner
                dest_map.insert(
                    dest_path.clone(),
                    DestRecord {
                        hash: disk_hash.clone(),
                        owner: Some(page_id.to_string()), // claim ownership for this run
                    },
                );
                continue;
            }

            // dest absent both in memory and on disk -> write and claim
            log::info!("Writing new asset at {dest_path:?} for page {page_id}, hash {hash}");
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).context("submit create dirs")?;
            }
            write_atomic_from_source(&input.src, &dest_path).context("write_atomic_from_source")?;
            dest_map.insert(
                dest_path.clone(),
                DestRecord {
                    hash: hash.clone(),
                    owner: Some(page_id.to_string()),
                },
            );
        }

        log::info!(
            "Computed {} asset dependencies for page {}",
            dependencies.len(),
            page_id
        );

        // Remove staled dependencies for this page, and update with new ones.
        if let Some(old_deps) = inner.page_dependencies.get_mut(page_id) {
            for old_path in old_deps.drain() {
                if !dependencies.contains(&old_path) {
                    log::info!("Removing stale asset dependency {old_path:?} for page {page_id}");
                    let _ = fs::remove_file(&old_path);
                }
            }
        }
        inner
            .page_dependencies
            .insert(page_id.to_string(), dependencies);

        Ok(())
    }
}

/// Helper to write source -> final_path atomically.
fn write_atomic_from_source(src: &AssetSource, final_path: &Path) -> io::Result<()> {
    let tmp = final_path.with_extension("tmp");
    let _ = fs::remove_file(&tmp);
    match src {
        AssetSource::Path(p) => {
            fs::copy(p, &tmp)?;
        }
        AssetSource::Bytes(b) => {
            let mut f = File::create(&tmp)?;
            f.write_all(b)?;
            f.sync_all()?;
        }
    }
    let _ = fs::remove_file(final_path);
    fs::rename(&tmp, final_path)?;
    Ok(())
}

/// Compute blake3 hash for AssetSource.
fn compute_hash_of_source(src: &AssetSource) -> io::Result<String> {
    match src {
        AssetSource::Path(p) => compute_hash_of_file(p),
        AssetSource::Bytes(b) => {
            let mut hasher = Hasher::new();
            hasher.update(b);
            Ok(hasher.finalize().to_hex().to_string())
        }
    }
}

/// Compute blake3 hash for a file (streaming).
fn compute_hash_of_file(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut hasher = Hasher::new();
    let mut buf = [0u8; 8 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}
