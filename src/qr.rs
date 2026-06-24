use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d,
    TextureDimension,
    TextureFormat,
};
use qrcode::QrCode;

pub fn generate_qr_image(text: &str) -> Image {
    let code = QrCode::new(text).unwrap();

    let qr = code.render::<image::Luma<u8>>().build();

    let width = qr.width();
    let height = qr.height();

    let mut rgba = Vec::with_capacity((width * height * 4) as usize);

    for pixel in qr.pixels() {
        let v = pixel[0];
        rgba.extend_from_slice(&[v, v, v, 255]);
    }

    Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &rgba,
        TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    )
}
