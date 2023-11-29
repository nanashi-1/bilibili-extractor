use crate::{
    error::Result,
    metadata::{NormalEpisodeMetadata, SeasonMetadata, SpecialEpisodeMetadata},
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
            "{} EP{:0>2}.mkv",
            $episode_metadata,
            $episode_video_path,
            $packager
        )
    };

    (special, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        package_episode!(
            copy,
            episode_name,
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
            "{} EP{:0>2}.mkv",
            $episode_metadata,
            $episode_video_path,
            $packager
        )
    };

    (special, $episode_metadata: expr, $episode_video_path: expr, $packager: expr) => {
        package_episode!(
            rename,
            episode_name,
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
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                $episode_metadata.$episode_ident, $episode_metadata.title
            ))?
            .as_ref()
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
    pub fn save_season(&self, season_metadata: &SeasonMetadata<impl AsRef<Path>>) -> Result<()> {
        season_metadata
            .normal_episodes
            .iter()
            .try_for_each(|e| self.save_normal_episode(e))?;

        season_metadata
            .special_episodes
            .iter()
            .try_for_each(|e| self.save_special_episode(e))?;

        Ok(())
    }

    /// Package a normal episode.
    pub fn save_normal_episode(
        &self,
        episode_metadata: &NormalEpisodeMetadata<impl AsRef<Path>>,
    ) -> Result<()> {
        let episode_video_path = get_episode_video_path!(episode, episode_metadata);
        create_dir_all(self.output_path.as_ref().join(&episode_metadata.title))?;

        if self.config.copy {
            package_epidode_by_copying!(normal, episode_metadata, episode_video_path, self);
        } else {
            package_epidode_by_moving!(normal, episode_metadata, episode_video_path, self);
        }

        Ok(())
    }

    /// Package a special episode.
    pub fn save_special_episode(
        &self,
        episode_metadata: &SpecialEpisodeMetadata<impl AsRef<Path>>,
    ) -> Result<()> {
        let episode_video_path = get_episode_video_path!(episode_name, episode_metadata);
        create_dir_all(self.output_path.as_ref().join(&episode_metadata.title))?;

        if self.config.copy {
            package_epidode_by_copying!(special, episode_metadata, episode_video_path, self);
        } else {
            package_epidode_by_moving!(special, episode_metadata, episode_video_path, self);
        }

        Ok(())
    }
}
