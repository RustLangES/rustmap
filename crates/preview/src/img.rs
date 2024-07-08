use std::time::{Duration, Instant};

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
) -> Duration {
    let time = Instant::now();
    let title = matter.title();
    let title = title
        .get(..47)
        .or_else(|| Some(title))
        .map(|s| {
            if s.len() >= 47 {
                format!("{s}...")
            } else {
                s.to_owned()
            }
        })
        .unwrap();
    let description = matter.description();
    let name = matter.name();

    let mut img = imageproc::drawing::draw_text(bg, BLACK, 24, 171, 42., bold_font, &title);

    // Description
    for (i, s) in chunked_string(description, 45, 8).iter().enumerate() {
        imageproc::drawing::draw_text_mut(
            &mut img,
            BLACK,
            56,
            317 + 28 + (34 * i as i32 + 1),
            34.,
            regular_font,
            &s,
        );
    }

    let out = format!("{out}/{name}.png");
    img.save_with_format(&out, image::ImageFormat::Png).unwrap();

    let time = time.elapsed();

    println!("Success ({t}): {name} at {out}", t = render_time(time));
    time
}

pub fn chunked_string(s: &str, line_width: usize, max_lines: usize) -> Vec<String> {
    let (lines, last) = s.split_whitespace().fold(
        (Vec::new(), String::new()),
        |(mut lines, mut current_line), word| {
            if current_line.len() + word.len() + (!current_line.is_empty() as usize) > line_width {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }

            if lines.len() == max_lines - 1 && !current_line.is_empty() {
                lines.push(current_line);
                return (lines, String::new());
            }

            (lines, current_line)
        },
    );
    lines
        .into_iter()
        .chain(std::iter::once(last))
        .take(max_lines)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::chunked_string;

    #[test]
    fn chunks() {
        let chunked = chunked_string("El Tipo de Dato `str` en Rust", 50, 8);
        assert_eq!(&chunked[..], &["El Tipo de Dato `str` en Rust"]);

        let chunked = chunked_string("Guía de configuracion para los Editores de Texto Más Comunes (Vs Code, Visual Studio, IntelliJ IDEA, Vim, Neovim, Sublime Text)", 50, 8);
        assert_eq!(
            &chunked[..],
            &[
                "Guía de configuracion para los Editores de Texto",
                "Más Comunes (Vs Code, Visual Studio, IntelliJ",
                "IDEA, Vim, Neovim, Sublime Text)",
            ]
        );
    }
}
