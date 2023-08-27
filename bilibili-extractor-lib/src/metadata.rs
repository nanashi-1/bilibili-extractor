use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Contains information inside a Bilibili JSON Entry file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonEntry {
    pub title: String,
    pub ep: JsonEntryEpisodeMetadata,
    pub type_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonEntryEpisodeMetadata {
    pub index_title: String,
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SeasonMetadata<P: AsRef<Path>> {
    pub title: String,
    pub path: Option<P>,
    pub normal_episodes: Vec<NormalEpisodeMetadata<P>>,
    pub special_episodes: Vec<SpecialEpisodeMetadata<P>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EpisodeMetadata<P: AsRef<Path>> {
    Normal(NormalEpisodeMetadata<P>),
    Special(SpecialEpisodeMetadata<P>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalEpisodeMetadata<P: AsRef<Path>> {
    pub episode: usize,
    pub path: Option<P>,
    pub type_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecialEpisodeMetadata<P: AsRef<Path>> {
    pub episode_name: String,
    pub path: Option<P>,
    pub type_tag: String,
}

impl<P: AsRef<Path>> SeasonMetadata<P> {
    pub fn new(entry: JsonEntry) -> Self {
        Self {
            title: entry.title,
            path: None,
            normal_episodes: vec![],
            special_episodes: vec![],
        }
    }

    pub fn set_path(self, path: P) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    pub fn add_episode(&mut self, episode: EpisodeMetadata<P>) {
        match episode {
            EpisodeMetadata::Normal(e) => self.normal_episodes.push(e),
            EpisodeMetadata::Special(e) => self.special_episodes.push(e),
        }
    }

    pub fn add_normal_episode(&mut self, episode: NormalEpisodeMetadata<P>) {
        self.normal_episodes.push(episode)
    }

    pub fn add_special_episode(&mut self, episode: SpecialEpisodeMetadata<P>) {
        self.special_episodes.push(episode)
    }
}

impl SeasonMetadata<PathBuf> {
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let json_entry = serde_json::from_str::<JsonEntry>(&read_to_string(
            path.as_ref()
                .read_dir()?
                .next()
                .ok_or(format!("No episodes found in {}", path.as_ref().display()))??
                .path()
                .join("entry.json"),
        )?)?;

        let mut season_metadata = Self::new(json_entry);

        for p in path.as_ref().read_dir()? {
            let episode_metadata = EpisodeMetadata::new_from_path(p?.path().join("entry.json"))?;

            season_metadata.add_episode(episode_metadata);
        }

        Ok(season_metadata)
    }
}

impl<P: AsRef<Path>> EpisodeMetadata<P> {
    pub fn new_from_path(path: P) -> Result<Self, Error> {
        let json = serde_json::from_str::<JsonEntry>(&read_to_string(path)?)?;

        Ok(json.into())
    }

    pub fn set_path(self, path: P) -> Self {
        match self {
            EpisodeMetadata::Normal(e) => Self::Normal(e.set_path(path)),
            EpisodeMetadata::Special(e) => Self::Special(e.set_path(path)),
        }
    }
}

impl<P: AsRef<Path>> NormalEpisodeMetadata<P> {
    pub fn set_path(self, path: P) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }
}

impl<P: AsRef<Path>> SpecialEpisodeMetadata<P> {
    pub fn set_path(self, path: P) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }
}

impl<P: AsRef<Path>> From<JsonEntry> for EpisodeMetadata<P> {
    fn from(val: JsonEntry) -> Self {
        match val.ep.index.parse::<usize>() {
            Ok(_) => EpisodeMetadata::Normal(val.try_into().unwrap()),
            Err(_) => EpisodeMetadata::Special(val.into()),
        }
    }
}

impl<P: AsRef<Path>> TryInto<NormalEpisodeMetadata<P>> for JsonEntry {
    type Error = Error;

    fn try_into(self) -> Result<NormalEpisodeMetadata<P>, Self::Error> {
        let episode = self.ep.index.parse()?;

        Ok(NormalEpisodeMetadata {
            episode,
            path: None,
            type_tag: self.type_tag,
        })
    }
}

impl<P: AsRef<Path>> From<JsonEntry> for SpecialEpisodeMetadata<P> {
    fn from(val: JsonEntry) -> Self {
        SpecialEpisodeMetadata {
            episode_name: val.ep.index,
            path: None,
            type_tag: val.type_tag,
        }
    }
}
