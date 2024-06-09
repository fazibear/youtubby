use anyhow::Result;

pub const INIT_SCRIPT: &str =
concat!(
    "document.addEventListener('load', () => {document.head.insertAdjacentHTML('beforeend',`<style>",
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stylesheet.css")),
    "</style>`);});\n",
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/scripts.js"))
);

pub const LOGO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/youtubby.png"));
pub const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png"));

pub fn get_image(data: &[u8]) -> Result<(Vec<u8>, u32, u32)> {
    let image = image::load_from_memory(data)?.into_rgba8();
    let width = image.dimensions().0;
    let height = image.dimensions().1;
    Ok((image.into_raw(), width, height))
}
