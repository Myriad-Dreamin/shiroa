use std::{fs::File, io::Read, path::Path};

use log::warn;

/// The `Theme` struct should be used instead of the static variables because
/// the `new()` method will look if the user has a theme directory in their
/// source folder and use the users theme instead of the default.
///
/// You should only ever use the static variables directly if you want to
/// override the user's theme with the defaults.
#[derive(Debug, PartialEq)]
pub struct Theme {
    pub index: Vec<u8>,
    pub typst_load_trampoline: Vec<u8>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            index: include_bytes!("../../themes/mdbook/index.hbs").to_vec(),
            typst_load_trampoline: include_bytes!("../../themes/mdbook/typst-load-trampoline.hbs")
                .to_vec(),
        }
    }
}

impl Theme {
    /// Creates a `Theme` from the given `theme_dir`.
    /// If a file is found in the theme dir, it will override the default
    /// version.
    pub fn new(theme_dir: &Path) -> Self {
        let mut theme = Self::default();

        // If the theme directory doesn't exist there's no point continuing...
        if !theme_dir.exists() || !theme_dir.is_dir() {
            panic!("Theme directory doesn't exist: {:?}", theme_dir);
        }

        // Check for individual files, if they exist copy them across
        {
            let files = vec![
                (theme_dir.join("index.hbs"), &mut theme.index),
                (
                    theme_dir.join("typst-load-trampoline.hbs"),
                    &mut theme.typst_load_trampoline,
                ),
            ];

            let load_with_warn = |filename: &Path, dest: &mut Vec<u8>| {
                if !filename.exists() {
                    // Don't warn if the file doesn't exist.
                    return false;
                }
                if let Err(e) = load_file_contents(filename, dest) {
                    warn!("Couldn't load custom file, {}: {}", filename.display(), e);
                    false
                } else {
                    true
                }
            };

            for (filename, dest) in files {
                load_with_warn(&filename, dest);
            }
        }

        // let fonts_dir = theme_dir.join("fonts");
        // ...

        theme
    }
}

/// Checks if a file exists, if so, the destination buffer will be filled with
/// its contents.
fn load_file_contents<P: AsRef<Path>>(filename: P, dest: &mut Vec<u8>) -> std::io::Result<()> {
    let filename = filename.as_ref();

    let mut buffer = Vec::new();
    File::open(filename)?.read_to_end(&mut buffer)?;

    // We needed the buffer so we'd only overwrite the existing content if we
    // could successfully load the file into memory.
    dest.clear();
    dest.append(&mut buffer);

    Ok(())
}
