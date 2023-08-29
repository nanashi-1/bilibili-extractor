use crate::{
    error::Result,
    metadata::{NormalEpisodeMetadata, SeasonMetadata, SpecialEpisodeMetadata},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{copy, create_dir_all, rename},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Packager<P: AsRef<Path>> {
    pub output_path: P,
    pub config: PackagerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PackagerConfig {
    pub copy: bool,
}

impl<P: AsRef<Path>> Packager<P> {
    pub fn new(output_path: P) -> Result<Self> {
        create_dir_all(&output_path)?;

        Ok(Self {
            output_path,
            config: Default::default(),
        })
    }

    pub fn set_config(self, config: PackagerConfig) -> Self {
        Self { config, ..self }
    }

    pub fn save_season(&self, season_metadata: SeasonMetadata<impl AsRef<Path>>) -> Result<()> {
        season_metadata
            .normal_episodes
            .iter()
            .map(|e| Ok(self.save_normal_episode(e)?))
            .collect::<Result<_>>()?;

        season_metadata
            .special_episodes
            .iter()
            .map(|e| Ok(self.save_special_episode(e)?))
            .collect::<Result<_>>()?;

        Ok(())
    }

    pub fn save_normal_episode(
        &self,
        episode_metadata: &NormalEpisodeMetadata<impl AsRef<Path>>,
    ) -> Result<()> {
        let episode_video_path = episode_metadata
            .path
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                episode_metadata.episode, episode_metadata.title
            ))?
            .as_ref()
            .join(&episode_metadata.type_tag)
            .join("episode.mkv");

        create_dir_all(self.output_path.as_ref().join(&episode_metadata.title))?;

        if self.config.copy {
            copy(
                episode_video_path,
                self.output_path
                    .as_ref()
                    .join(&episode_metadata.title)
                    .join(format!(
                        "{} EP{:0>2}.mkv",
                        episode_metadata.title, episode_metadata.episode
                    )),
            )?;
        } else {
            rename(
                episode_video_path,
                self.output_path
                    .as_ref()
                    .join(&episode_metadata.title)
                    .join(format!(
                        "{} EP{:0>2}.mkv",
                        episode_metadata.title, episode_metadata.episode
                    )),
            )?;
        }

        Ok(())
    }

    pub fn save_special_episode(
        &self,
        episode_metadata: &SpecialEpisodeMetadata<impl AsRef<Path>>,
    ) -> Result<()> {
        let episode_video_path = episode_metadata
            .path
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                episode_metadata.episode_name, episode_metadata.title
            ))?
            .as_ref()
            .join(&episode_metadata.type_tag)
            .join("episode.mkv");

        if self.config.copy {
            copy(
                episode_video_path,
                self.output_path
                    .as_ref()
                    .join(&episode_metadata.title)
                    .join(format!(
                        "{} {}.mkv",
                        episode_metadata.title, episode_metadata.episode_name
                    )),
            )?;
        } else {
            rename(
                episode_video_path,
                self.output_path
                    .as_ref()
                    .join(&episode_metadata.title)
                    .join(format!(
                        "{} {}.mkv",
                        episode_metadata.title, episode_metadata.episode_name
                    )),
            )?;
        }

        Ok(())
    }
}
