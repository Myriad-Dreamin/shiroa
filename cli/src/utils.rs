use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::{fs, io};

use reflexo_typst::error::prelude::*;
use reflexo_typst::TypstSystemWorld;
use regex::Regex;
use tokio::runtime::Builder;

/// Replaces multiple consecutive whitespace characters with a single space
/// character.
pub fn collapse_whitespace(text: &str) -> Cow<'_, str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s\s+").unwrap());
    RE.replace_all(text, " ")
}

pub fn async_continue<F: std::future::Future<Output = ()>>(f: F) -> ! {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f);

    #[allow(unreachable_code)]
    {
        unreachable!("The async command must exit the process.");
    }
}

pub fn exit_with_error<E: std::error::Error>(err: E) -> ! {
    clap::Error::raw(
        clap::error::ErrorKind::ValueValidation,
        format!("shiroa error: {err}"),
    )
    .exit()
}

pub trait UnwrapOrExit<T> {
    fn unwrap_or_exit(self) -> T;
}

impl<T, E: std::error::Error> UnwrapOrExit<T> for Result<T, E> {
    fn unwrap_or_exit(self) -> T {
        self.map_err(exit_with_error).unwrap()
    }
}

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_exit()
}

pub fn make_absolute_from(path: &Path, relative_to: impl FnOnce() -> PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        relative_to().join(path)
    }
}

pub fn make_absolute(path: &Path) -> PathBuf {
    make_absolute_from(path, current_dir)
}

/// <https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust>
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn create_dirs<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        return Ok(());
    }

    fs::create_dir_all(path).map_err(error_once_map!("create_dirs"))
}

pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        if !path.is_file() {
            return Err(error_once!("Cannot write file: not a file at path", path: path.display()));
        }
        // todo: check mtime
        // check content
        if fs::read(path).map_err(error_once_map!("write_file: read"))? == contents.as_ref() {
            return Ok(());
        }
    }

    fs::write(path, contents.as_ref()).map_err(error_once_map!("write_file: write"))
}

pub fn copy_dir_embedded(src: &include_dir::Dir, dst: &Path) -> Result<()> {
    // Create all the subdirectories in here (but not their files yet)
    for dir in src.dirs() {
        create_dirs(dst.join(dir.path()))?;
        // Recurse for this directory
        copy_dir_embedded(dir, dst)?;
    }
    for entry in src.files() {
        if let Some(dir) = dst.join(entry.path()).parent() {
            create_dirs(dir)?;
        }
        let t = dst.join(entry.path());
        write_file(t, entry.contents())?;
    }
    Ok(())
}

fn release_packages_inner(world: &mut TypstSystemWorld, pkg: include_dir::Dir, no_override: bool) {
    fn get_string(v: &toml::Value) -> &str {
        match v {
            toml::Value::String(table) => table,
            _ => unreachable!(),
        }
    }

    let manifest = pkg.get_file("typst.toml").unwrap().contents_utf8().unwrap();
    let manifest: toml::Table = toml::from_str(manifest).unwrap();

    let pkg_info = match manifest.get("package").unwrap() {
        toml::Value::Table(table) => table,
        _ => unreachable!(),
    };

    let name = get_string(pkg_info.get("name").unwrap());
    let version = get_string(pkg_info.get("version").unwrap());

    let pkg_dirname = format!("{name}/{version}");

    let local_path = world.registry.local_path().unwrap();
    let pkg_link_target = make_absolute(&local_path.join("preview").join(&pkg_dirname));

    if pkg_link_target.exists() {
        eprintln!("package {pkg_dirname} already exists");
        if no_override {
            return;
        }
    }

    std::fs::create_dir_all(pkg_link_target.parent().unwrap()).unwrap();
    copy_dir_embedded(&pkg, &pkg_link_target).unwrap();
}

pub fn release_packages(world: &mut TypstSystemWorld, pkg: include_dir::Dir) {
    release_packages_inner(world, pkg, false);
}
