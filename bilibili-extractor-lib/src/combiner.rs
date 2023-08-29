use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, ExitStatus},
};

use crate::{error::Error, metadata::NormalEpisodeMetadata, subtitle::SubtitleType};

pub trait Combinable {
    fn combine(
        &self,
        aubtitle_path: impl AsRef<Path>,
        subtitle_language: &str,
        subtitle_type: SubtitleType,
        output_path: impl AsRef<Path>,
    ) -> Result<ExitStatus, Error>;
}

impl<P: AsRef<Path>> Combinable for NormalEpisodeMetadata<P> {
    fn combine(
        &self,
        subtitle_path: impl AsRef<Path>,
        subtitle_language: &str,
        subtitle_type: SubtitleType,
        output_path: impl AsRef<Path>,
    ) -> Result<ExitStatus, Error> {
        let video_path = self
            .path
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                self.episode, self.title
            ))?
            .as_ref()
            .join(&self.type_tag)
            .join("video.m4a");

        let audio_path = self
            .path
            .as_ref()
            .unwrap()
            .as_ref()
            .join(&self.type_tag)
            .join("audio.m4a");

        let mut binding = Command::new("ffmpeg");
        binding
            .arg("-y")
            .args(["-hide_banner", "-loglevel", "error"]) // silent operation
            .args([OsStr::new("-i"), video_path.as_os_str()])
            .args([OsStr::new("-i"), audio_path.as_os_str()]);

        match subtitle_type {
            SubtitleType::Hard => Ok(binding
                .args([
                    "-vf",
                    &format!("subtitles={}", subtitle_path.as_ref().display()),
                ])
                .arg(output_path.as_ref().as_os_str())
                .status()?),
            SubtitleType::Soft => Ok(binding
                .args([OsStr::new("-i"), subtitle_path.as_ref().as_os_str()])
                .args(["-map", "0"])
                .args(["-map", "1:a:0"])
                .args(["-map", "2"])
                .args(["-metadata:s:s:0", &format!("language={subtitle_language}")])
                .args(["-codec", "copy"])
                .status()?),
        }
    }
}
