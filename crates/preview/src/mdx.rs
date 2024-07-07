use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FrontMatter {
    title: String,
    description: String,
    pub draft: bool,
}

impl FrontMatter {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub fn from_file(path: &str, matter: &Matter<YAML>) -> FrontMatter {
    let content = std::fs::read_to_string(path).unwrap();
    matter.parse(&content).data.unwrap().deserialize().unwrap()
}
