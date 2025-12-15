use std::{collections::BTreeMap, net::SocketAddr, path::PathBuf};

use reflexo_typst::{
    path::unix_slash,
    vfs::{notify::NotifyMessage, FilesystemEvent, FsProvider},
    watch_deps, ImmutStr, TypstSystemWorld, WorldDeps,
};
use tokio::sync::{broadcast, mpsc};

use crate::{project::Project, render::SearchRenderer, tui, tui_hint, tui_info};

impl Project {
    pub(crate) async fn watch(
        &mut self,
        // active_set: Arc<Mutex<HashMap<ImmutStr, usize>>>,
        mut hb_rx: mpsc::UnboundedReceiver<ServeEvent>,
        tx: broadcast::Sender<WatchSignal>,
        addr: Option<SocketAddr>,
    ) {
        let _ = self.build();
        let (dep_tx, dep_rx) = mpsc::unbounded_channel();
        let (fs_tx, mut fs_rx) = mpsc::unbounded_channel();
        tokio::spawn(watch_deps(dep_rx, move |event| {
            fs_tx.send(event).unwrap();
        }));

        let need_compile = self.need_compile();
        let finish = |world: &mut TypstSystemWorld| {
            // Notify the new file dependencies.
            let mut deps = vec![];
            world.iter_dependencies(&mut |dep| {
                if let Ok(x) = world.file_path(dep).and_then(|e| e.to_err()) {
                    deps.push(x.into())
                }
            });

            // TODO: Add static asset files to watched dependencies

            tui_info!("Watching {} files for changes...", deps.len());
            let _ = dep_tx.send(NotifyMessage::SyncDependency(Box::new(deps)));

            if need_compile {
                comemo::evict(10);
                world.evict_source_cache(30);
                world.evict_vfs(60);
            }

            if let Some(addr) = &addr {
                tui_hint!("Server started at http://{addr}");
            }
        };

        let mut snap = self.tr.snapshot();
        let mut world = snap.world.clone();
        // first report.
        finish(&mut world);

        let mut active_files: BTreeMap<ImmutStr, usize> = BTreeMap::new();
        loop {
            enum WatchEvent {
                Fs(FilesystemEvent),
                Serve(ServeEvent),
            }

            let event = tokio::select! {
                event = fs_rx.recv() => {
                    match event {
                        Some(e) => WatchEvent::Fs(e),
                        None => break,
                    }
                }
               Some(c) = hb_rx.recv() => WatchEvent::Serve(c),
            };

            // todo: reset_snapshot looks not good

            let is_heartbeat = matches!(event, WatchEvent::Serve(ServeEvent::HoldPath(..)));
            match event {
                WatchEvent::Fs(event) => {
                    self.tr.reset_snapshot();
                    self.tr.universe_mut().increment_revision(|verse| {
                        verse.vfs().notify_fs_event(event);
                    });

                    let _ = tui::clear();
                    let _ = self.build_meta();

                    snap = self.tr.snapshot();
                    world = snap.world.clone();
                }
                WatchEvent::Serve(ServeEvent::HoldPath(path, inc)) => {
                    let path = if path.as_ref() == "/" || path.is_empty() {
                        if let Some(path) = self.chapters.first().and_then(|f| f.path.clone()) {
                            path
                        } else {
                            continue;
                        }
                    } else if path.ends_with(".html") {
                        let path = path.trim_start_matches('/');
                        let typ_path = PathBuf::from(path);
                        unix_slash(&typ_path.with_extension("typ")).into()
                    } else {
                        path
                    };

                    let active_files = &mut active_files;
                    let mut changed = false;
                    if inc {
                        *active_files.entry(path).or_insert_with(|| {
                            changed = true;
                            0
                        }) += 1;
                    } else {
                        let count = active_files.entry(path);
                        // erase if the count is 1, otherwise decrement
                        match count {
                            std::collections::btree_map::Entry::Occupied(mut e) => {
                                if *e.get() > 1 {
                                    *e.get_mut() -= 1;
                                } else {
                                    changed = true;
                                    e.remove();
                                }
                            }
                            std::collections::btree_map::Entry::Vacant(_) => {}
                        }
                    }

                    if !changed {
                        // No changes, skip recompilation
                        continue;
                    }

                    let _ = tui::clear();
                    tui_info!("Recompiling changed chapters: {active_files:?}");

                    let _ = self.build_meta();
                }
            }

            // todo: blocking?
            let _ = self.compile_once(&active_files, SearchRenderer::new());

            if !is_heartbeat {
                let _ = tx.send(WatchSignal::Reload);
            }
            finish(&mut world);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServeEvent {
    HoldPath(ImmutStr, bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchSignal {
    Reload,
}
