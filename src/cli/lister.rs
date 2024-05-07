use crate::colorer::Colorer;
use bilibili_extractor_lib::metadata::EpisodeId;
use bilibili_extractor_lib::metadata::EpisodeMetadata;
use bilibili_extractor_lib::metadata::SeasonMetadata;

pub struct Lister;

impl Lister {
    pub fn list_seasons(&self, seasons: &[SeasonMetadata]) {
        seasons.iter().enumerate().for_each(|(i, s)| {
            println!(
                "{} {}",
                format!("{}.) Title:", i + 1).color_as_success(),
                s.title
            );

            #[cfg(debug_assertions)]
            {
                println!("    {} {:?}", "Path:".color_as_success(), s.path);
                println!(
                    "    {} {:?}",
                    "Episode Count:".color_as_success(),
                    s.episodes.len()
                );
            }

            let normal_episodes: Vec<_> = s
                .episodes
                .iter()
                .filter(|e| matches!(e.episode, EpisodeId::Normal(_)))
                .collect();

            let special_episodes: Vec<_> = s
                .episodes
                .iter()
                .filter(|e| matches!(e.episode, EpisodeId::Special(_)))
                .collect();

            match (normal_episodes.is_empty(), special_episodes.is_empty()) {
                (true, true) => println!("{}", "    No Episodes!!".color_as_error()),
                (false, true) => {
                    println!("{}", "    Episodes:".color_as_success());
                    self.list_normal_episodes(normal_episodes);
                }
                (true, false) => {
                    println!("{}", "    Episodes:".color_as_success());
                    self.list_special_episodes(special_episodes);
                }
                _ => {
                    println!("{}", "    Normal Episodes:\n".color_as_success());
                    self.list_normal_episodes(normal_episodes);
                    println!("{}", "    Special Episodes:\n".color_as_success());
                    self.list_special_episodes(special_episodes);
                }
            }
        })
    }

    pub fn list_normal_episodes(&self, episodes: Vec<&EpisodeMetadata>) {
        #[cfg(not(debug_assertions))]
        episodes
            .iter()
            .for_each(|e| println!("        {} EP{:0>2}", e.title, e.episode));

        #[cfg(debug_assertions)]
        episodes.iter().for_each(|e| {
            println!(
                "        {} {}, Path: {:?}",
                e.title,
                e.episode.get_full_display(),
                e.path
            )
        });
    }

    pub fn list_special_episodes(&self, episodes: Vec<&EpisodeMetadata>) {
        #[cfg(not(debug_assertions))]
        episodes
            .iter()
            .for_each(|e| println!("        {} {}", e.title, e.episode_name));

        #[cfg(debug_assertions)]
        episodes.iter().for_each(|e| {
            println!(
                "        {} {}, Path: {:?}",
                e.title,
                e.episode.get_full_display(),
                e.path
            )
        });
    }
}
