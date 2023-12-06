use crate::colorer::Colorer;
use bilibili_extractor_lib::metadata::{
    NormalEpisodeMetadata, SeasonMetadata, SpecialEpisodeMetadata,
};
use std::{convert::AsRef, path::Path};

pub struct Lister;

impl Lister {
    pub fn list_seasons(&self, seasons: &[SeasonMetadata<impl AsRef<Path>>]) {
        seasons.iter().enumerate().for_each(|(i, s)| {
            println!(
                "{} {}",
                format!("{}.) Title:", i + 1).color_as_success(),
                s.title
            );

            #[cfg(debug_assertions)]
            {
                println!(
                    "    {} {:?}",
                    "Path:".color_as_success(),
                    s.path.as_ref().unwrap().as_ref()
                );
                println!(
                    "    {} {:?}",
                    "Episode Count:".color_as_success(),
                    s.normal_episodes.len() + s.special_episodes.len()
                );
            }

            if s.normal_episodes.is_empty() && s.special_episodes.is_empty() {
                println!("{}", "    No Episodes!!".color_as_error());
            } else if !s.normal_episodes.is_empty() && s.special_episodes.is_empty() {
                println!("{}", "    Episodes:".color_as_success());
                self.list_normal_episodes(&s.normal_episodes);
            } else if s.normal_episodes.is_empty() && !s.special_episodes.is_empty() {
                println!("{}", "    Episodes:".color_as_success());
                self.list_special_episodes(&s.special_episodes);
            } else {
                println!("{}", "    Normal Episodes:\n".color_as_success());
                self.list_normal_episodes(&s.normal_episodes);
                println!("{}", "    Special Episodes:\n".color_as_success());
                self.list_special_episodes(&s.special_episodes);
            }

            println!()
        })
    }

    pub fn list_normal_episodes(&self, episodes: &[NormalEpisodeMetadata<impl AsRef<Path>>]) {
        #[cfg(not(debug_assertions))]
        episodes
            .iter()
            .for_each(|e| println!("        {} EP{:0>2}", e.title, e.episode));

        #[cfg(debug_assertions)]
        episodes.iter().for_each(|e| {
            println!(
                "        {} EP{:0>2}, Path: {:?}",
                e.title,
                e.episode,
                e.path.as_ref().unwrap().as_ref()
            )
        });
    }

    pub fn list_special_episodes(&self, episodes: &[SpecialEpisodeMetadata<impl AsRef<Path>>]) {
        #[cfg(not(debug_assertions))]
        episodes
            .iter()
            .for_each(|e| println!("        {} {}", e.title, e.episode_name));

        #[cfg(debug_assertions)]
        episodes.iter().for_each(|e| {
            println!(
                "        {} {}, Path: {:?}",
                e.title,
                e.episode_name,
                e.path.as_ref().unwrap().as_ref()
            )
        });
    }
}
