// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    build_macos_app()
}

fn build_macos_app() -> std::io::Result<()> {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build = env::var("PROFILE").unwrap();
    let out_dir = Path::new(&root).join("target").join(&build);

    let bin_name = "youtubby";
    let app_name = "Youtubby";

    let app_dir = Path::new(&out_dir).join(&app_name).with_extension("app");

    let contents_dir = Path::new(&app_dir).join("Contents");
    let macos_dir = Path::new(&app_dir).join("MacOS");
    let resources_dir = Path::new(&contents_dir).join("Resources");

    fs::remove_dir_all(&app_dir)?;
    fs::create_dir_all(&contents_dir)?;
    fs::create_dir_all(&macos_dir)?;
    fs::create_dir_all(&resources_dir)?;

    let src_app_bin = Path::new(&out_dir).join(&bin_name);
    let dst_app_bin = Path::new(&macos_dir).join(&bin_name);
    fs::copy(src_app_bin, dst_app_bin)?;

    let src_icon = Path::new(&root)
        .join("assets")
        .join("youtubby")
        .with_extension("icns");
    let dst_icon = Path::new(&resources_dir)
        .join("AppIcon")
        .with_extension("icns");
    fs::copy(src_icon, dst_icon)?;

    let src_icon = Path::new(&root)
        .join("assets")
        .join("Info")
        .with_extension("plist");
    let dst_icon = Path::new(&contents_dir)
        .join("Info")
        .with_extension("plist");
    fs::copy(src_icon, dst_icon)?;

    println!("cargo:warning={}{:?}", "MacOS App: ", app_dir);

    Ok(())
}

// fn build_macos_dmg() -> std::io::Result<()> {
//     let dmg_dir = Path::new(&out_dir).join(&app_name).with_extension("dmg");
//
//     Ok(())
// }
