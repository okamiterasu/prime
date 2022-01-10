use std::path::Path;

use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Export
{
    export_warframes: Vec<Warframe>,
    // export_abilities: Value
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Warframe
{
    pub unique_name: String,
    pub name: String,
    pub product_category: String
}

pub fn parse_from_file(path: &Path) -> std::io::Result<Vec<Warframe>>
{
    let file_contents = std::fs::read_to_string(path)?;
    let escaped: String = file_contents
        .replace(r"\r", "")
        .replace(&['\r', '\n'][..], "");
    let parsed: Export = serde_json::from_str(&escaped)?;
    let frames = parsed.export_warframes;
    Ok(frames)
}