use ab_glyph::FontRef;
use gray_matter::engine::YAML;
use gray_matter::Matter;

use crate::img::generate_preview;

mod img;
mod mdx;

const BOLD_FONT: &[u8] = include_bytes!("../assets/WorkSans-Bold.ttf");
const REGULAR_FONT: &[u8] = include_bytes!("../assets/WorkSans-Regular.ttf");

fn main() {
    let mut args = std::env::args().skip(1);

    let bg = args
        .next()
        .expect("The first argument must be the background preset file");

    let out = args
        .next()
        .expect("The seconds argument must be the output directory");

    let files = if args.len() > 2 {
        args.collect::<Vec<String>>()
    } else {
        let dir = args.next().expect(
            "The third argument must be the directory with the content or files to be processed",
        );

        // get all files recursively on dir
        walkdir::WalkDir::new(&dir)
            .sort_by_file_name()
            .into_iter()
            .flat_map(|d| {
                let Ok(d) = d else {
                    return None;
                };
                if d.file_type().is_file() && d.path().extension().is_some_and(|e| e != "yml") {
                    return d.path().to_str().map(|d| d.to_string());
                }
                None
            })
            .collect()
    };

    let matter = Matter::<YAML>::new();

    if std::fs::metadata(&out).is_err() {
        println!("Creating directory");
        std::fs::create_dir_all(&out).unwrap();
    }

    let bold_font = FontRef::try_from_slice(BOLD_FONT).unwrap();
    let regular_font = FontRef::try_from_slice(REGULAR_FONT).unwrap();
    let bg = image::open(bg).unwrap().into_rgba8();

    files.iter().for_each(|f| {
        generate_preview(
            &bg,
            &bold_font,
            &regular_font,
            &out,
            mdx::from_file(f.as_str(), &matter),
        )
    })
}
