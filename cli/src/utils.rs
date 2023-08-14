use std::path::{Path, PathBuf};

use tokio::runtime::Builder;

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
        format!("typst-book error: {}", err),
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
