use std::process::{Command, ExitStatus};

/// This function will merge the specified video, audio, and subtitle into one file.
///
/// # Errors
///
/// This function will return an error if:
///
/// * `ffmpeg` fails to execute.
/// * The video, audio, or subtitle files do not exist.
/// * The video, audio, and subtitle files are not in the correct formats.
///
/// # Examples
///
/// ```
/// use bilibili_extractor::ffmpeg_controller::merge;
///
/// merge(
///    "tests/ffmpeg_encode_assets/video.m4s",
///    "tests/ffmpeg_encode_assets/audio.m4s",
///    "tests/ffmpeg_encode_assets/subtitle.ass",
///    "tests/ffmpeg_encode_assets/episode.mkv",
///     false
/// )
/// .unwrap();
/// ```
pub fn merge(
    video_path: &str,
    audio_path: &str,
    subtitle_path: &str,
    output_path: &str,
    hard_subtitle: bool,
) -> std::io::Result<ExitStatus> {
    let mut binding = Command::new("ffmpeg");
    binding
        .arg("-y")
        .args(["-hide_banner", "-loglevel", "error"]) // silent operation
        .args(["-i", video_path])
        .args(["-i", audio_path]);

    if hard_subtitle {
        return binding
            .args(["-vf", format!("subtitles={}", subtitle_path).as_str()])
            .arg(output_path)
            .status();
    }

    binding
        .args(["-i", subtitle_path])
        .args(["-map", "0"])
        .args(["-map", "1:a:0"])
        .args(["-map", "2"])
        .args(["-codec", "copy"])
        .arg(output_path)
        .status()
}
