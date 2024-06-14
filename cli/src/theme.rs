use std::{fs::File, io::Read, path::Path};

use log::warn;
use typst_ts_core::{error::prelude::*, ImmutPath};

use crate::utils::{self, copy_dir_embedded, write_file};
use include_dir::include_dir;

#[derive(Debug, PartialEq)]
pub enum EmbeddedThemeAsset {
    MdBook,
}

#[derive(Debug, PartialEq)]
pub enum ThemeAsset {
    Static(EmbeddedThemeAsset),
    Dir(ImmutPath),
}

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

    asset: ThemeAsset,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            index: include_bytes!("../../themes/mdbook/index.hbs").to_vec(),
            typst_load_trampoline: include_bytes!("../../themes/mdbook/typst-load-trampoline.hbs")
                .to_vec(),
            asset: ThemeAsset::Static(EmbeddedThemeAsset::MdBook),
        }
    }
}

impl Theme {
    /// Creates a `Theme` from the given `theme_dir`.
    /// If a file is found in the theme dir, it will override the default
    /// version.
    pub fn new(theme_dir: &Path) -> ZResult<Self> {
        let mut theme = Self {
            asset: ThemeAsset::Dir(theme_dir.into()),
            ..Default::default()
        };

        // If the theme directory doesn't exist there's no point continuing...
        if !theme_dir.exists() || !theme_dir.is_dir() {
            return Err(error_once!(
                "Theme directory doesn't exist",
                theme_dir: theme_dir.display(),
            ));
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

        Ok(theme)
    }

    pub fn is_static(&self) -> bool {
        matches!(self.asset, ThemeAsset::Static(_))
    }

    pub fn copy_assets(&self, dest_dir: &Path) -> ZResult<()> {
        if !dest_dir.exists() {
            log::debug!(
                "{} does not exist, creating the directory",
                dest_dir.display()
            );
            utils::create_dirs(dest_dir)?;
        }

        match &self.asset {
            ThemeAsset::Static(EmbeddedThemeAsset::MdBook) => {
                copy_dir_embedded(
                    include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/css"),
                    dest_dir.join("css"),
                )?;
                copy_dir_embedded(
                    include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/css"),
                    dest_dir.join("FontAwesome/css"),
                )?;
                copy_dir_embedded(
                    include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/fonts"),
                    dest_dir.join("FontAwesome/fonts"),
                )?;
                write_file(
                    dest_dir.join("index.js"),
                    include_bytes!("../../themes/mdbook/index.js"),
                )?;
            }
            ThemeAsset::Dir(theme_dir) => {
                utils::copy_dir_all(theme_dir, dest_dir)
                    .map_err(error_once_map!("copy_theme_directory"))?;
            }
        }

        Ok(())
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
