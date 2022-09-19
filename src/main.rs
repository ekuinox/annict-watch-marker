mod animetick_exported_data;
mod annict;

use crate::{
    animetick_exported_data::{read_animations_by_directory, read_animations_by_file},
    annict::{episode::GetEpisodesResponse, work::GetWorksResponse, AnnictClient},
};
use anyhow::{bail, ensure, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct App {
    directory: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::try_parse()?;
    let annict_token = std::env::var("ANNICT_TOKEN")?;
    let animations = if app.directory.is_file() {
        read_animations_by_file(&app.directory)?
    } else {
        read_animations_by_directory(&app.directory)?
    };

    let reg = regex::Regex::new(r"#(\d+)\s+(.+)")?;

    let animations = animations
        .into_iter()
        .map(|animation| {
            (
                animation.name,
                animation
                    .episodes
                    .into_iter()
                    .flat_map(|animetick_exported_data::Episode(subtitle, watched)| {
                        let caps = match reg.captures(&subtitle) {
                            Some(caps) => caps,
                            None => return None,
                        };
                        let number = match caps.get(1) {
                            Some(n) => n,
                            None => return None,
                        };
                        let subtitle = match caps.get(2) {
                            Some(s) => s,
                            None => return None,
                        };
                        let number = match number.as_str().parse::<f64>() {
                            Ok(n) => n,
                            Err(_) => return None,
                        };
                        Some((number, subtitle.as_str().to_string(), watched))
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let client = AnnictClient::new(annict_token);
    for (title, animetick_episodes) in animations {
        println!("- start: {title}");
        let r = exec_animation(&client, &title, &animetick_episodes).await;
        println!("-> {}", if r.is_ok() { "ok" } else { "err" });
        if let Err(e) = &r {
            eprintln!("-> {e}");
        }
    }

    Ok(())
}

async fn exec_animation(
    client: &AnnictClient,
    title: &str,
    animetick_episodes: &[(f64, String, Option<bool>)],
) -> Result<()> {
    let GetWorksResponse { works, .. } = client.get_works_by_title(&title).await?;
    let work = match works.into_iter().find(|work| work.title == title) {
        Some(w) => w,
        None => bail!("-> cannot found on annict"),
    };

    let status_kind = if animetick_episodes
        .iter()
        .all(|(_, _, watched)| watched.unwrap_or(false))
    {
        "watched" // 視聴完了
    } else {
        "on_hold" // 中断
    };
    client.status(work.id, status_kind).await?;

    let GetEpisodesResponse { episodes, .. } = client.get_episodes_by_work_id(work.id).await?;
    let episodes = episodes
        .into_iter()
        .map(|episode| (episode.number, episode.title, episode.id))
        .collect::<Vec<_>>();

    let episodes = {
        let mut episodes = animetick_episodes
            .into_iter()
            .flat_map(|(animetick_number, _, watched)| {
                let episode = episodes.iter().find(|(number, _, _)| {
                    number
                        .map(|number| *animetick_number == number)
                        .unwrap_or(false)
                });
                episode.map(|(number, title, id)| (number, title, id, watched))
            })
            .collect::<Vec<_>>();
        episodes.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap_or(std::cmp::Ordering::Equal));
        episodes
    };

    let episodes = episodes
        .into_iter()
        .filter_map(|(_, _, id, watched)| {
            watched.and_then(|watched| if watched { id.into() } else { None })
        })
        .collect::<Vec<_>>();
    let results = futures::future::join_all(episodes.into_iter().map(|episode_id| {
        let episode_id = *episode_id;
        async move {
            std::thread::sleep(std::time::Duration::from_millis(1));
            let r = client.record(episode_id).await;
            if let Err(e) = &r {
                eprintln!("--> {episode_id}, {e}");
            }
            r
        }
    }))
    .await;

    ensure!(results.into_iter().all(|result| result.is_ok()));

    Ok(())
}
