use crate::{
    error::Result,
    metadata::{NormalEpisodeMetadata, SpecialEpisodeMetadata},
    subtitle::SubtitleType,
};
use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, ExitStatus},
};

macro_rules! get_file {
    ($file_name: literal, $episode_ident: ident, $combinable: expr) => {
        $combinable
            .path
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                $combinable.$episode_ident, $combinable.title
            ))?
            .as_ref()
            .join(&$combinable.type_tag)
            .join($file_name)
    };
}

pub trait Combinable {
    /// Combine the audio, video, and subtitle using `ffmpeg`.
    fn combine(
        &self,
        subtitle_path: impl AsRef<Path>,
        subtitle_language: &str,
        subtitle_type: SubtitleType,
    ) -> Result<ExitStatus>;
}

impl<P: AsRef<Path>> Combinable for NormalEpisodeMetadata<P> {
    fn combine(
        &self,
        subtitle_path: impl AsRef<Path>,
        subtitle_language: &str,
        subtitle_type: SubtitleType,
    ) -> Result<ExitStatus> {
        let video_path = get_file!("video.m4s", episode, self);

        let audio_path = get_file!("audio.m4s", episode, self);

        let output_path = get_file!("episode.mkv", episode, self);

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
                .arg(output_path)
                .status()?),
            SubtitleType::Soft => Ok(binding
                .args([OsStr::new("-i"), subtitle_path.as_ref().as_os_str()])
                .args(["-map", "0"])
                .args(["-map", "1:a:0"])
                .args(["-map", "2"])
                .args(["-metadata:s:s:0", &format!("language={subtitle_language}")])
                .args(["-disposition:s:s:0", "default"])
                .args(["-codec", "copy"])
                .arg(output_path)
                .status()?),
        }
    }
}

impl<P: AsRef<Path>> Combinable for SpecialEpisodeMetadata<P> {
    fn combine(
        &self,
        subtitle_path: impl AsRef<Path>,
        subtitle_language: &str,
        subtitle_type: SubtitleType,
    ) -> Result<ExitStatus> {
        let video_path = get_file!("video.m4s", episode_name, self);

        let audio_path = get_file!("audio.m4s", episode_name, self);

        let output_path = get_file!("episode.mkv", episode_name, self);

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
                .arg(output_path)
                .status()?),
            SubtitleType::Soft => Ok(binding
                .args([OsStr::new("-i"), subtitle_path.as_ref().as_os_str()])
                .args(["-map", "0"])
                .args(["-map", "1:a:0"])
                .args(["-map", "2"])
                .args(["-metadata:s:s:0", &format!("language={subtitle_language}")])
                .args(["-codec", "copy"])
                .arg(output_path)
                .status()?),
        }
    }
}
