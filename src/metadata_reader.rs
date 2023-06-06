use serde_derive::Deserialize;
use serde_json::from_str;
use std::{
    ffi::OsStr,
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
struct Entry {
    title: String,
    ep: EpisodeEntry,
    type_tag: String,
}

/// Contains the episode id, title, and index.
#[derive(Deserialize, Debug)]
struct EpisodeEntry {
    index_title: String,
    index: String,
}

#[derive(Clone, Debug)]
pub struct EpisodeMetadata {
    pub title: String,
    pub index: String,
    pub path: PathBuf,
}

/// Contains the season title and episodes.
#[derive(Clone, Debug)]
pub struct SeasonMetadata {
    pub title: String,
    pub type_tag: String,
    pub episodes: Vec<EpisodeMetadata>,
}

impl SeasonMetadata {
    fn new(title: String, type_tag: String) -> Self {
        Self {
            title,
            type_tag,
            episodes: Default::default(),
        }
    }
}

impl EpisodeEntry {
    fn into<P: AsRef<OsStr>>(self, path: P) -> EpisodeMetadata {
        EpisodeMetadata {
            title: self.index_title,
            index: self.index,
            path: Path::new(&path).into(),
        }
    }
}

/// This function gets the episode metadata from the specified episode directory.
///
/// # Errors
///
/// This function returns an error if the `entry.json` doesn't exist or not a valid Bilibili JSON entry file.
///
/// # Examples
///
/// ```
/// use bilibili_extractor::metadata_reader::{get_episode_metadata, get_season_metadata};
///
/// let episode_metadata =
/// get_episode_metadata("tests/metadata_parsing_assets/s_36534/377500").unwrap();
///
/// assert_eq!(
///     "EP 1 - Say what you want; famous games are usually fun",
///     episode_metadata.title
/// );
/// assert_eq!("1", episode_metadata.index);
/// ```
pub fn get_episode_metadata<P: AsRef<OsStr>>(episode_path: P) -> std::io::Result<EpisodeMetadata> {
    let string = read_to_string(Path::new(&episode_path).join("entry.json"))?;
    Ok(from_str::<Entry>(&string)?.ep.into(episode_path))
}

/// This function gets the metadata of the season from the specified season directory.
///
/// # Errors
///
/// This function returns an error if the directory is empty or [`get_episode_metadata()`] fails.
///
/// # Examples
///
/// ```
/// use bilibili_extractor::metadata_reader::{get_episode_metadata, get_season_metadata};
///
/// let season_metadata = get_season_metadata("tests/metadata_parsing_assets/s_36534").unwrap();
///
/// assert_eq!("Bottom-tier Character Tomozaki ", season_metadata.title);
/// assert_eq!(14, season_metadata.episodes.len());
/// ```
pub fn get_season_metadata<P: AsRef<OsStr>>(season_path: P) -> std::io::Result<SeasonMetadata> {
    let season_directory = Path::new(&season_path);

    let mut season_metadata = if let Some(f) = season_directory.read_dir()?.next() {
        let season_metadata_path = f?.path().join("entry.json");
        let string = read_to_string(season_metadata_path)?;
        let entry = from_str::<Entry>(&string)?;

        SeasonMetadata::new(entry.title, entry.type_tag)
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Directory is empty!"));
    };

    season_directory
        .read_dir()?
        .map(|e| {
            let episode_metadata = get_episode_metadata(e?.path())?;
            season_metadata.episodes.push(episode_metadata);
            Ok(())
        })
        .collect::<Result<Vec<()>, Error>>()?;

    Ok(season_metadata)
}
