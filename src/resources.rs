use std::path::{Path};

use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Export
{
    export_resources: Vec<Resource>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource
{
    pub unique_name: String,
    pub name: String,
}

pub fn parse_from_file(path: &Path) -> std::io::Result<Vec<Resource>>
{
    let file_contents = std::fs::read_to_string(path)?;
    let escaped = file_contents
        .replace(r"\r", "")
        .replace(&['\r', '\n'][..], "");
    let parsed: Export = serde_json::from_str(&escaped)?;
    let frames = parsed.export_resources;
    Ok(frames)
}