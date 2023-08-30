use std::path::Path;

use bilibili_extractor_lib::metadata::{
    DownloadFolder, NormalEpisodeMetadata, SeasonMetadata, SpecialEpisodeMetadata,
};

use crate::{colorer::Colorer, Context};

pub struct Lister {
    context: Context,
}

impl Lister {
    pub fn list_seasons(download_directory: DownloadFolder<impl AsRef<Path>>) {
        download_directory
            .seasons
            .iter()
            .enumerate()
            .for_each(|(i, s)| {
                println!(
                    "{} {}",
                    format!("{}.) Title:", i + 1).color_as_success(),
                    s.title
                );

                if s.normal_episodes.is_empty() && s.special_episodes.is_empty() {
                    println!("{}", "    No Episodes!!".color_as_error());
                } else if !s.normal_episodes.is_empty() && s.special_episodes.is_empty() {
                    println!("{}", "    Episodes:".color_as_success());
                    Self::list_normal_episodes(&s.normal_episodes);
                } else if s.normal_episodes.is_empty() && !s.special_episodes.is_empty() {
                    println!("{}", "    Episodes:".color_as_success());
                    Self::list_special_episodes(&s.special_episodes);
                } else {
                    println!("{}", "    Normal Episodes:\n".color_as_success());
                    Self::list_normal_episodes(&s.normal_episodes);
                    println!("{}", "    Special Episodes:\n".color_as_success());
                    Self::list_special_episodes(&s.special_episodes);
                }

                println!()
            })
    }

    pub fn list_normal_episodes(episodes: &Vec<NormalEpisodeMetadata<impl AsRef<Path>>>) {
        episodes
            .iter()
            .for_each(|e| println!("        {} EP{:0>2}", e.title, e.episode))
    }

    pub fn list_special_episodes(episodes: &Vec<SpecialEpisodeMetadata<impl AsRef<Path>>>) {
        episodes
            .iter()
            .for_each(|e| println!("        {} {}", e.title, e.episode_name))
    }
}
