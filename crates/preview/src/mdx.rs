use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FrontMatter {
    title: String,
    description: String,
    pub draft: bool,

    #[serde(skip)]
    name: String,
}

impl FrontMatter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub fn from_file(path: &str, matter: &Matter<YAML>) -> FrontMatter {
    let content = std::fs::read_to_string(path).unwrap();
    let raw = matter.parse(&content).data.unwrap().deserialize().unwrap();

    FrontMatter {
        name: path
            .replace("./", "")
            .split('/')
            .map(|p| p.split('.').skip(1).collect::<String>())
            .collect::<Vec<String>>()
            .join("-"),
        ..raw
    }
}
