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
