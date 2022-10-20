use bevy::prelude::*;
use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

pub struct ImageGenOptions {
    pub premultiplied_alpha: PremultipliedAlpha,
}

pub enum PremultipliedAlpha {
    None,
    SRGB,
    Linear,
}

pub fn image_gen(options: ImageGenOptions) -> Image {
    let mut rgba: Vec<[f32; 4]> = vec![];
    let width = 100;
    let height = 100;
    for y in 0..height {
        for x in 0..width {
            let xx = x as f32 / width as f32;
            let yy = y as f32 / height as f32;
            let distance_to_center =
                (Vec2::new(0.5, 0.5).distance(Vec2::new(xx, yy)) * 2.5).clamp(0.0, 1.0);
            let edged_circle = 1.0 - distance_to_center.powf(20.0);
            let a = edged_circle;
            let mut color = Color::rgb_u8(254, 215, 191);
            if matches!(options.premultiplied_alpha, PremultipliedAlpha::SRGB) {
                color.set_r(color.r() * a);
                color.set_g(color.g() * a);
                color.set_b(color.b() * a);
            }
            let mut linear = color.as_linear_rgba_f32();
            if matches!(options.premultiplied_alpha, PremultipliedAlpha::Linear) {
                linear[0] *= a;
                linear[1] *= a;
                linear[2] *= a;
            }
            rgba.push([linear[0], linear[1], linear[2], a]);
        }
    }
    Image::new(
        Extent3d {
            depth_or_array_layers: 1,
            width,
            height,
        },
        TextureDimension::D2,
        Vec::from(unsafe {
            std::slice::from_raw_parts(rgba.as_ptr() as *const u8, rgba.len() * 16)
        }),
        TextureFormat::Rgba32Float,
    )
}
