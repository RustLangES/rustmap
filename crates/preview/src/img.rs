use std::time::Instant;

use ab_glyph::FontRef;
use image::{Rgba, RgbaImage};

use crate::mdx::FrontMatter;
use crate::render_time;

const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

pub fn generate_preview(
    bg: &RgbaImage,
    bold_font: &FontRef,
    regular_font: &FontRef,
    out: &str,
    matter: FrontMatter,
) {
    let time = Instant::now();
    let title = matter.title();
    let description = matter.description();
    let name = matter.name();

    let img = imageproc::drawing::draw_text(bg, BLACK, 24, 171, 42., bold_font, title);
    let img = imageproc::drawing::draw_text(&img, BLACK, 56, 317, 34., regular_font, description);

    let out = format!("{out}/{name}.png");
    img.save_with_format(&out, image::ImageFormat::Png).unwrap();

    println!(
        "Success ({time}): {name} at {out}",
        time = render_time(time)
    );
}
