pub const INIT_SCRIPT: &str =
    concat!(
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/scripts.js")),
    "window.addEventListener('load', () => {document.head.insertAdjacentHTML('beforeend',`<style>",
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stylesheet.css")),
    "</style>`);});"
);

pub const LOGO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/youtubby.png"));
pub const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png"));

pub fn get_image(data: &[u8]) -> (Vec<u8>, u32, u32) {
    let image = image::load_from_memory(data).unwrap().into_rgba8();
    let width = image.dimensions().0;
    let height = image.dimensions().1;
    (image.into_raw(), width, height)
}

// static ICON_IMAGE: DynamicImage = image::load_from_memory(ICON_DATA).unwrap();
//
// pub static ICON_RGBA: ImageBuffer<Rgba<u8>, Vec<u8>> = ICON_IMAGE.into_rgba8();
// pub const ICON: Vec<u8> = ICON_RGBA.into_raw();
// pub const ICON_WIDTH: u32 = ICON_RGBA.dimensions().0;
// pub const ICON_HEIGHT: u32 = ICON_RGBA.dimensions().1;
