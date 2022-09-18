mod animetick_exported_data;

use crate::animetick_exported_data::{read_animations_by_directory, read_animations_by_file};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct App {
    directory: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::try_parse()?;
    let animations = if app.directory.is_file() {
        read_animations_by_file(&app.directory)?
    } else {
        read_animations_by_directory(&app.directory)?
    };
    dbg!(&animations);

    Ok(())
}
