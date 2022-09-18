mod animetick_exported_data;

use animetick_exported_data::read_animations_by_directory;
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
    let animations = read_animations_by_directory(&app.directory)?;
    dbg!(&animations);

    Ok(())
}
