use crate::colorer::Colorer;
use crate::{create_spinner, Context};
use bilibili_extractor_lib::combiner::Combinable;
use bilibili_extractor_lib::error::Result;
use bilibili_extractor_lib::metadata::{EpisodeId, EpisodeMetadata, SeasonMetadata};
use bilibili_extractor_lib::subtitle::{JsonSubtitle, SubtitleFormat};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rsubs_lib::srt::SRTFile;
use rsubs_lib::vtt::VTTFile;
use std::fs::{read_to_string, rename};
use std::str::FromStr;

pub struct Compiler {
    context: Context,
}

impl Compiler {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub fn compile_seasons(&self, seasons: &[SeasonMetadata]) -> Result<()> {
        seasons.iter().try_for_each(|s| self.compile_season(s))?;

        Ok(())
    }

    pub fn compile_season(&self, season_metadata: &SeasonMetadata) -> Result<()> {
        #[cfg(debug_assertions)]
        println!(
            "{} Season Name: {:?}, Season Path: {:?}, Episode Count: {:?}\n",
            "DEBUG:".color_as_warning(),
            season_metadata.title,
            season_metadata.path,
            season_metadata.episodes.len()
        );

        let normal_episodes: Vec<&EpisodeMetadata> = season_metadata
            .episodes
            .iter()
            .filter(|e| matches!(e.episode, EpisodeId::Normal(_)))
            .collect();
        let special_episodes: Vec<&EpisodeMetadata> = season_metadata
            .episodes
            .iter()
            .filter(|e| matches!(e.episode, EpisodeId::Special(_)))
            .collect();

        self.compile_normal_episodes(normal_episodes)?;
        self.compile_special_episodes(special_episodes)?;

        self.context.packager.save_season(season_metadata)?;

        Ok(())
    }

    pub fn compile_normal_episodes(&self, episodes: Vec<&EpisodeMetadata>) -> Result<()> {
        match self.context.is_parallel {
            true => episodes
                .par_iter()
                .try_for_each(|e| self.compile_normal_episode(e))?,
            false => episodes
                .iter()
                .try_for_each(|e| self.compile_normal_episode(e))?,
        };

        Ok(())
    }

    pub fn compile_special_episodes(&self, episodes: Vec<&EpisodeMetadata>) -> Result<()> {
        match self.context.is_parallel {
            true => episodes
                .par_iter()
                .try_for_each(|e| self.compile_special_episode(e))?,
            false => episodes
                .iter()
                .try_for_each(|e| self.compile_special_episode(e))?,
        };

        Ok(())
    }

    pub fn compile_normal_episode(&self, episode: &EpisodeMetadata) -> Result<()> {
        #[cfg(debug_assertions)]
        println!(
            "{} Episode Type: \"Normal\", Episode: {:?}, Episode Path: {:?}, Subtitle Format: \"{:?}\"",
            "DEBUG:".color_as_warning(),
            episode.episode,
            episode.path,
            SubtitleFormat::get_episode_subtitle_type(episode, &self.context.language)?
        );

        let mut spinner = match self.context.is_parallel {
            true => create_spinner("Compiling episodes in parallel..."),
            false => create_spinner(&format!(
                "Compiling {} EP{:0>2}...",
                episode.title, episode.episode
            )),
        };

        let subtitle_path = episode.get_subtitle_path(&self.context.language)?;
        let binding = episode.path.join("subtitle.ass");
        let output_subtitle_path = binding.to_str().ok_or("Path is not valid Unicode")?;

        match SubtitleFormat::get_episode_subtitle_type(episode, &self.context.language)? {
            SubtitleFormat::Json => JsonSubtitle::new_from_path(subtitle_path)?
                .to_ssa()
                .to_file(output_subtitle_path),
            SubtitleFormat::Ssa => rename(subtitle_path, output_subtitle_path),
            SubtitleFormat::Srt => SRTFile::from_str(&read_to_string(subtitle_path)?)?
                .to_ass()
                .to_file(output_subtitle_path),
            SubtitleFormat::Vtt => VTTFile::from_str(&read_to_string(subtitle_path)?)?
                .to_ass()
                .to_file(output_subtitle_path),
        }?;

        episode.combine(
            output_subtitle_path,
            &self.context.language,
            self.context.subtitle_type,
        )?;

        spinner.stop_and_persist(
            &"✔".color_as_success(),
            format!("Compiled {} EP{:0>2}!", episode.title, episode.episode).color_as_success(),
        );

        #[cfg(debug_assertions)]
        println!();

        Ok(())
    }

    pub fn compile_special_episode(&self, episode: &EpisodeMetadata) -> Result<()> {
        #[cfg(debug_assertions)]
        println!(
            "{} Episode Type: \"Special\", Episode Name: {:?}, Episode Path: {:?}, Subtitle Format: \"{:?}\"",
            "DEBUG:".color_as_warning(),
            episode.episode,
            episode.path,
            SubtitleFormat::get_episode_subtitle_type(episode, &self.context.language)?
        );

        let mut spinner = match self.context.is_parallel {
            true => create_spinner("Compiling episodes in parallel..."),
            false => create_spinner(&format!("Compiling {} {}", episode.title, episode.episode)),
        };

        let subtitle_path = episode.get_subtitle_path(&self.context.language)?;
        let binding = episode.path.join("subtitle.ass");
        let output_subtitle_path = binding.to_str().ok_or("Path is not valid Unicode")?;

        match SubtitleFormat::get_episode_subtitle_type(episode, &self.context.language)? {
            SubtitleFormat::Json => JsonSubtitle::new_from_path(subtitle_path)?
                .to_ssa()
                .to_file(output_subtitle_path),
            SubtitleFormat::Ssa => rename(subtitle_path, output_subtitle_path),
            SubtitleFormat::Srt => SRTFile::from_str(&read_to_string(subtitle_path)?)?
                .to_ass()
                .to_file(output_subtitle_path),
            SubtitleFormat::Vtt => VTTFile::from_str(&read_to_string(subtitle_path)?)?
                .to_ass()
                .to_file(output_subtitle_path),
        }?;

        episode.combine(
            output_subtitle_path,
            &self.context.language,
            self.context.subtitle_type,
        )?;

        spinner.stop_and_persist(
            &"✔".color_as_success(),
            format!("Compiled {} {}!", episode.title, episode.episode).color_as_success(),
        );

        #[cfg(debug_assertions)]
        println!();

        Ok(())
    }
}
