//! [animetick](http://animetick.net)から
//! [animetick-exporter-bookmarklet](https://github.com/ekuinox/animetick-exporter-bookmarklet)を使って
//! ダウンロードしたファイルを扱う

use anyhow::Result;
use serde::Deserialize;
use std::fs::{read_dir, File};
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Episode(pub String, pub Option<bool>);

#[derive(Deserialize, Debug)]
pub struct Animation {
    pub name: String,
    pub episodes: Vec<serde_json::Value>,
}

pub type AnimetickExportedDataAnimations = Vec<Animation>;

pub fn read_animations_by_file(path: &Path) -> Result<AnimetickExportedDataAnimations> {
    let file = File::open(path)?;
    let animations = serde_json::from_reader(file)?;
    Ok(animations)
}

pub fn read_animations_by_directory(path: &Path) -> Result<AnimetickExportedDataAnimations> {
    let dir = read_dir(path)?;
    let animations = dir
        .into_iter()
        .flatten()
        .flat_map(|entry| read_animations_by_file(&entry.path()))
        .flatten()
        .collect();
    Ok(animations)
}
