//! Some code are from typst/typst/crates/cli

#![allow(unused)]

use core::fmt;
use std::io::{self, IsTerminal, Write};
use std::sync::OnceLock;

use termcolor::{Color, ColorChoice, ColorSpec, WriteColor};

const PREFIX_LEN: usize = 12;

/// Returns a handle to the optionally colored terminal output.
pub fn init_out(choice: clap::ColorChoice) -> &'static termcolor::StandardStream {
    static OUT: OnceLock<termcolor::StandardStream> = OnceLock::new();

    OUT.get_or_init(|| {
        termcolor::StandardStream::stderr(match choice {
            clap::ColorChoice::Auto if std::io::stderr().is_terminal() => ColorChoice::Auto,
            clap::ColorChoice::Always => ColorChoice::Always,
            _ => ColorChoice::Never,
        })
    })
}

/// Returns a handle to the optionally colored terminal output.
pub fn out() -> &'static termcolor::StandardStream {
    init_out(clap::ColorChoice::Auto)
}

/// Clears the entire screen.
pub fn clear() -> io::Result<()> {
    let out = out();

    // We don't want to clear anything that is not a TTY.
    if out.supports_color() {
        let mut stream = out.lock();
        // Clear the screen and then move the cursor to the top left corner.
        write!(stream, "\x1B[2J\x1B[1;1H")?;
        stream.flush()?;
    }

    Ok(())
}

#[macro_export]
macro_rules! tui_error {
    (h$prefix:literal, $( $arg:tt )*) => { $crate::tui_msg!(Error, $prefix, $($arg)*) };
    ($( $arg:tt )*) => { $crate::tui_msg!(Error, "Error:", $($arg)*) };
}
#[macro_export]
macro_rules! tui_warn {
    (h$prefix:literal, $( $arg:tt )*) => { $crate::tui_msg!(Warn, $prefix, $($arg)*) };
    ($( $arg:tt )*) => { $crate::tui_msg!(Warn, "Warn:", $($arg)*) };
}
#[macro_export]
macro_rules! tui_info {
    (h$prefix:literal, $( $arg:tt )*) => { $crate::tui_msg!(Info, $prefix, $($arg)*) };
    ($( $arg:tt )*) => { $crate::tui_msg!(Info, "Info:", $($arg)*) };
}
#[macro_export]
macro_rules! tui_hint {
    (h$prefix:literal, $( $arg:tt )*) => { $crate::tui_msg!(Hint, $prefix, $($arg)*) };
    ($( $arg:tt )*) => { $crate::tui_msg!(Hint, "Hint:", $($arg)*) };
}
#[macro_export]
macro_rules! tui_msg {
    ($level:ident, $prefix:literal, $($arg:tt)*) => { $crate::tui::msg($crate::tui::Level::$level, $prefix, format_args!($($arg)*)) };
}

pub enum Level {
    Error,
    Warn,
    Info,
    Hint,
}

pub fn msg(level: Level, prefix: &str, msg: fmt::Arguments) {
    let mut out = out().lock();

    let header = ColorSpec::new().set_bold(true).set_intense(true).clone();
    let header = match level {
        Level::Error => header.clone().set_fg(Some(Color::Red)).clone(),
        Level::Warn => header.clone().set_fg(Some(Color::Yellow)).clone(),
        Level::Info => header.clone().set_fg(Some(Color::Green)).clone(),
        Level::Hint => header.clone().set_fg(Some(Color::Cyan)).clone(),
    };

    // todo: handle errors in these not important io.
    let _ = out.set_color(&header);
    let _ = write!(out, "{prefix:>PREFIX_LEN$}");
    let _ = out.reset();
    let _ = write!(out, " {msg}");
    let _ = writeln!(out);
}
