use gray_matter::engine::YAML;
use gray_matter::Matter;

mod mdx;

fn main() {
    let mut args = std::env::args().skip(1);

    let _out = args
        .next()
        .expect("The first argument must be the output directory");

    let files = if args.len() > 1 {
        args.collect::<Vec<String>>()
    } else {
        let dir = args.next().expect(
            "The second argument must be the directory with the content or files to be processed",
        );

        // get all files recursively on dir
        walkdir::WalkDir::new(&dir)
            .sort_by_file_name()
            .into_iter()
            .flat_map(|d| d.map(|d| d.path().to_str().unwrap().to_string()).ok())
            .collect()
    };

    let matter = Matter::<YAML>::new();
}