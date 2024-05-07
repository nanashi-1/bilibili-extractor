use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

/// Contains information inside a Bilibili JSON Entry file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonEntry {
    pub title: String,
    pub ep: JsonEntryEpisodeMetadata,
    pub type_tag: String,
}

/// Contains information about the episode. It can be found inside a Bilibili JSON file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonEntryEpisodeMetadata {
    pub index_title: String,
    pub index: String,
}

/// The download folder of Bilibili. Contains all the seasons downloaded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DownloadFolder {
    pub seasons: Vec<SeasonMetadata>,
}

/// Contains information of the entire season, including its episode, both normal and special.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SeasonMetadata {
    pub title: String,
    pub path: PathBuf,
    pub episodes: Vec<EpisodeMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EpisodeId {
    Normal(usize),
    Special(String),
}

/// Contains information of the normal episode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpisodeMetadata {
    pub title: String,
    pub episode: EpisodeId,
    pub path: PathBuf,
    pub type_tag: String,
}

impl DownloadFolder {
    /// Creates a `DownloadFolder` from path.
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let mut seasons = vec![];

        for p in path.as_ref().read_dir()? {
            let season_metadata = match SeasonMetadata::new_from_path(p?.path()) {
                Ok(v) => v,
                Err(_) => continue,
            };

            seasons.push(season_metadata);
        }

        seasons.sort();

        Ok(Self { seasons })
    }
}

impl SeasonMetadata {
    /// Creates a `SeasonMetadata` from path.
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let json_entry = serde_json::from_str::<JsonEntry>(&read_to_string(
            path.as_ref()
                .read_dir()?
                .next()
                .ok_or(format!("No episodes found in {}", path.as_ref().display()))??
                .path()
                .join("entry.json"),
        )?)?;

        let mut season_metadata = Self {
            title: json_entry.title,
            path: path.as_ref().into(),
            episodes: vec![],
        };

        for p in path.as_ref().read_dir()? {
            let episode_metadata = EpisodeMetadata::new_from_path(p?.path())?;

            season_metadata.episodes.push(episode_metadata);
        }

        season_metadata.episodes.sort();

        Ok(season_metadata)
    }
}

impl EpisodeId {
    pub fn get_full_display(&self) -> String {
        match self {
            EpisodeId::Normal(v) => format!("Episode {:0>2}", v),
            EpisodeId::Special(v) => v.to_string(),
        }
    }
}

impl Display for EpisodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EpisodeId::Normal(v) => write!(f, "{:0>2}", v),
            EpisodeId::Special(v) => write!(f, "{}", v),
        }
    }
}

impl EpisodeMetadata {
    /// Create an episode metadata from path.
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let json =
            serde_json::from_str::<JsonEntry>(&read_to_string(path.as_ref().join("entry.json"))?)?;

        Ok(Self::from(json).set_path(path.as_ref().into()))
    }

    pub fn get_subtitle_path(&self, subtitle_language: &str) -> Result<PathBuf> {
        Ok(self
            .path
            .join(subtitle_language)
            .read_dir()?
            .next()
            .ok_or("Subtitle directory is empty.")??
            .path())
    }

    pub fn set_path(mut self, path: PathBuf) -> Self {
        self.path = path;

        self
    }
}

impl From<JsonEntry> for EpisodeMetadata {
    fn from(val: JsonEntry) -> Self {
        match val.ep.index.parse::<usize>() {
            Ok(e) => EpisodeMetadata {
                title: val.title,
                episode: EpisodeId::Normal(e),
                path: Default::default(),
                type_tag: val.type_tag,
            },
            Err(_) => EpisodeMetadata {
                title: val.title,
                episode: EpisodeId::Special(val.ep.index),
                path: Default::default(),
                type_tag: val.type_tag,
            },
        }
    }
}
