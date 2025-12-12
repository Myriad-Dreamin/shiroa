use include_dir::include_dir;
use reflexo_typst::TypstSystemWorld;

use crate::utils::{copy_dir_embedded, make_absolute};

pub fn release_builtin_packages(world: &mut TypstSystemWorld) {
    release_packages(
        world,
        include_dir!("$CARGO_MANIFEST_DIR/../packages/shiroa"),
    );
    release_packages(
        world,
        include_dir!("$CARGO_MANIFEST_DIR/../themes/starlight"),
    );
    release_packages(world, include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook"));
}

fn release_packages(world: &mut TypstSystemWorld, pkg: include_dir::Dir) {
    release_packages_inner(world, pkg, false);
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
