use crate::{
    error::Result,
    metadata::{EpisodeId, EpisodeMetadata, SeasonMetadata},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{copy, create_dir_all, rename},
    path::Path,
};

macro_rules! package_episode {
    ($package_method: ident, $episode_ident: ident, $name_format: literal, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        $package_method(
            $episode_video_path,
            $packager
                .output_path
                .as_ref()
                .join(&$episode_metadata.title)
                .join(format!(
                    $name_format,
                    $episode_metadata.title, $episode_metadata.$episode_ident
                )),
        )?
    };
}

macro_rules! package_epidode_by_copying {
    (normal, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        package_episode!(
            copy,
            episode,
            "{} EP{}.mkv",
            $episode_metadata,
            $episode_video_path,
            $packager
        )
    };

    (special, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        package_episode!(
            copy,
            episode,
            "{} {}.mkv",
            $episode_metadata,
            $episode_video_path,
            $packager
        )
    };
}

macro_rules! package_epidode_by_moving {
    (normal, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        package_episode!(
            rename,
            episode,
            "{} EP{}.mkv",
            $episode_metadata,
            $episode_video_path,
            $packager
        )
    };

    (special, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        package_episode!(
            rename,
            episode,
            "{} {}.mkv",
            $episode_metadata,
            $episode_video_path,
            $packager
        )
    };
}

macro_rules! get_episode_video_path {
    ($episode_ident: ident, $episode_metadata: expr) => {
        $episode_metadata
            .path
            .join(&$episode_metadata.type_tag)
            .join("episode.mkv")
    };
}

/// Packages seasons and episodes.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Packager<P: AsRef<Path>> {
    pub output_path: P,
    pub config: PackagerConfig,
}

/// Config used for packaging.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PackagerConfig {
    pub copy: bool,
}

impl<P: AsRef<Path>> Packager<P> {
    /// Create a new `Packager`.
    pub fn new(output_path: P) -> Result<Self> {
        create_dir_all(&output_path)?;

        Ok(Self {
            output_path,
            config: Default::default(),
        })
    }

    /// Set config for `Packager`.
    pub fn set_config(self, config: PackagerConfig) -> Self {
        Self { config, ..self }
    }

    /// Package a season.
    pub fn save_season(&self, season_metadata: &SeasonMetadata) -> Result<()> {
        season_metadata
            .episodes
            .iter()
            .try_for_each(|e| self.save_episode(e))?;

        Ok(())
    }

    /// Package episode.
    pub fn save_episode(&self, episode_metadata: &EpisodeMetadata) -> Result<()> {
        let episode_video_path = get_episode_video_path!(episode, episode_metadata);
        create_dir_all(self.output_path.as_ref().join(&episode_metadata.title))?;

        if self.config.copy {
            match episode_metadata.episode {
                EpisodeId::Normal(_) => {
                    package_epidode_by_copying!(normal, episode_metadata, episode_video_path, self)
                }
                EpisodeId::Special(_) => {
                    package_epidode_by_copying!(special, episode_metadata, episode_video_path, self)
                }
            };
        } else {
            match episode_metadata.episode {
                EpisodeId::Normal(_) => {
                    package_epidode_by_moving!(normal, episode_metadata, episode_video_path, self)
                }
                EpisodeId::Special(_) => {
                    package_epidode_by_moving!(special, episode_metadata, episode_video_path, self)
                }
            };
        }

        Ok(())
    }
}
