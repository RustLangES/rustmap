use crate::mdx::FrontMatter;

pub fn generate_preview(matter: FrontMatter, out: &str) {
    let _title = matter.title();
    let _description = matter.description();
    let _name = matter.name();
}
