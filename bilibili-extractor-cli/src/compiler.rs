use crate::Context;
use bilibili_extractor_lib::combiner::Combinable;
use bilibili_extractor_lib::error::Result;
use bilibili_extractor_lib::metadata::{
    NormalEpisodeMetadata, SeasonMetadata, SpecialEpisodeMetadata,
};
use bilibili_extractor_lib::subtitle::{JsonSubtitle, SubtitleFormat};
use rsubs_lib::srt::SRTFile;
use rsubs_lib::vtt::VTTFile;
use std::convert::AsRef;
use std::fs::{read_to_string, rename};
use std::path::Path;
use std::str::FromStr;

pub struct Compiler<P: AsRef<Path>> {
    context: Context<P>,
}

impl<P: AsRef<Path>> Compiler<P> {
    pub fn new(context: Context<P>) -> Self {
        Self { context }
    }

    pub fn compile_seasons(&self, seasons: &Vec<SeasonMetadata<impl AsRef<Path>>>) -> Result<()> {
        seasons.iter().try_for_each(|s| self.compile_season(s))?;

        Ok(())
    }

    pub fn compile_season(&self, season_metadata: &SeasonMetadata<impl AsRef<Path>>) -> Result<()> {
        self.compile_normal_episodes(&season_metadata.normal_episodes)?;
        self.compile_special_episodes(&season_metadata.special_episodes)?;

        self.context.packager.save_season(season_metadata)?;

        Ok(())
    }

    pub fn compile_normal_episodes(
        &self,
        episodes: &Vec<NormalEpisodeMetadata<impl AsRef<Path>>>,
    ) -> Result<()> {
        episodes
            .iter()
            .try_for_each(|e| self.compile_normal_episode(e))?;

        Ok(())
    }

    pub fn compile_special_episodes(
        &self,
        episodes: &Vec<SpecialEpisodeMetadata<impl AsRef<Path>>>,
    ) -> Result<()> {
        episodes
            .iter()
            .try_for_each(|e| self.compile_special_episode(e))?;

        Ok(())
    }

    pub fn compile_normal_episode(
        &self,
        episode: &NormalEpisodeMetadata<impl AsRef<Path>>,
    ) -> Result<()> {
        let subtitle_path = episode.get_subtitle_path(self.context.language)?;
        let binding = episode.path.as_ref().unwrap().as_ref().join("subtitle.ass");
        let output_subtitle_path = binding.to_str().ok_or("Path is not valid Unicode")?;

        match SubtitleFormat::get_normal_episode_subtitle_type(episode, self.context.language)? {
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
            self.context.language,
            self.context.subtitle_type,
        )?;

        Ok(())
    }

    pub fn compile_special_episode(
        &self,
        episode: &SpecialEpisodeMetadata<impl AsRef<Path>>,
    ) -> Result<()> {
        let subtitle_path = episode.get_subtitle_path(self.context.language)?;
        let binding = episode.path.as_ref().unwrap().as_ref().join("subtitle.ass");
        let output_subtitle_path = binding.to_str().ok_or("Path is not valid Unicode")?;

        match SubtitleFormat::get_special_episode_subtitle_type(episode, self.context.language)? {
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
            self.context.language,
            self.context.subtitle_type,
        )?;

        Ok(())
    }
}
