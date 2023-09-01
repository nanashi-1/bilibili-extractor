use crate::{
    error::{Error, Result},
    metadata::{EpisodeMetadata, NormalEpisodeMetadata, SpecialEpisodeMetadata},
};
use rsubs_lib::{
    srt::{SRTFile, SRTLine},
    ssa::{SSAEvent, SSAFile, SSAStyle},
    util::{
        color::{
            Alignment, Color,
            ColorType::{self, SSAColor},
        },
        time::Time,
    },
    vtt::{VTTFile, VTTLine, VTTStyle},
    Subtitle,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

/// Contains information inside a Bilibili JSON subtitle.
///
/// # Convert to other subtitle format
///
/// ```
/// use bilibili_extractor_lib::subtitle_converter::{JsonSubtitle, JsonSubtitleBody};
///
/// let json_subtitle = JsonSubtitle {
///     body: vec![JsonSubtitleBody {
///         from: 0.,
///         to: 1.,
///         content: "Subtitle".into(),
///     }],
/// };
///
/// println!("{}", json_subtitle.to_ssa().to_string())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct JsonSubtitle {
    pub body: Vec<JsonSubtitleBody>,
}

/// Lines inside a Bilibili JSON subtitle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct JsonSubtitleBody {
    pub from: f32,
    pub to: f32,
    pub content: String,
}

/// Either a softsub or hardsub.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SubtitleType {
    Hard,
    #[default]
    Soft,
}

/// Format of the subtitle. Though Bilibili only uses SSA and JSON.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SubtitleFormat {
    #[default]
    Json,
    Ssa,
    Srt,
    Vtt,
}

impl JsonSubtitle {
    /// Fetch the json subtitle of an episode.
    pub fn new_from_episode(
        episode: &EpisodeMetadata<impl AsRef<Path>>,
        subtitle_language: &str,
    ) -> Result<Self> {
        match episode {
            EpisodeMetadata::Normal(e) => Self::new_from_normal_episode(e, subtitle_language),
            EpisodeMetadata::Special(e) => Self::new_from_special_episode(e, subtitle_language),
        }
    }

    /// Fetch the json subtitle of a normal episode.
    pub fn new_from_normal_episode(
        episode: &NormalEpisodeMetadata<impl AsRef<Path>>,
        subtitle_language: &str,
    ) -> Result<Self> {
        let subtitle_path = episode
            .path
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                episode.episode, episode.title
            ))?
            .as_ref()
            .join(subtitle_language)
            .read_dir()?
            .next()
            .ok_or("Subtitle directory is empty.")??
            .path();

        Self::new_from_path(subtitle_path)
    }

    /// Fetch the json subtitle of a special episode.
    pub fn new_from_special_episode(
        episode: &SpecialEpisodeMetadata<impl AsRef<Path>>,
        subtitle_language: &str,
    ) -> Result<Self> {
        let subtitle_path = episode
            .path
            .as_ref()
            .ok_or(format!(
                "{} of {} doesn't have a path.",
                episode.episode_name, episode.title
            ))?
            .as_ref()
            .join(subtitle_language)
            .read_dir()?
            .next()
            .ok_or("Subtitle directory is empty.")??
            .path();

        Self::new_from_path(subtitle_path)
    }

    /// Create a `JsonSubtitle` from path.
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let json_string = fs::read_to_string(path)?;

        Ok(serde_json::from_str(&json_string)?)
    }

    /// Convert to `Subtitle`.
    pub fn to_subtitle(self) -> Subtitle {
        self.into()
    }

    /// Convert to `SSAFile`.
    pub fn to_ssa(self) -> SSAFile {
        self.into()
    }

    /// Convert to `SRTFile`.
    pub fn to_srt(self) -> SRTFile {
        self.into()
    }

    /// Convert to `VTTFile`.
    pub fn to_vtt(self) -> VTTFile {
        self.into()
    }
}

impl From<JsonSubtitle> for Subtitle {
    fn from(value: JsonSubtitle) -> Self {
        let mut ass_info = HashMap::new();
        ass_info.insert("Title".into(), "Bilibili Subtitle".into());
        ass_info.insert("ScriptType".into(), "v4.00+".into());
        ass_info.insert("WrapStyle".into(), "0".into());
        ass_info.insert("ScaledBorderAndShadow".into(), "yes".into());
        ass_info.insert("YCbCr Matrix".into(), "TV.601".into());
        ass_info.insert("PlayResX".into(), "1920".into());
        ass_info.insert("PlayResY".into(), "1080".into());

        let ass_styles = SSAStyle {
            name: "Default".into(),
            fontname: "Noto Sans".into(),
            fontsize: 100.,
            firstcolor: SSAColor(Color {
                a: 0,
                r: 255,
                g: 255,
                b: 255,
            }),
            secondcolor: SSAColor(Color {
                r: 0,
                g: 255,
                b: 255,
                a: 0,
            }),
            outlinecolor: SSAColor(Color {
                r: 8,
                g: 34,
                b: 0,
                a: 0,
            }),
            backgroundcolor: SSAColor(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 127,
            }),
            bold: false,
            italic: false,
            unerline: false,
            strikeout: false,
            scalex: 100.,
            scaley: 100.,
            spacing: 0.,
            angle: 0.,
            borderstyle: 1,
            outline: 5.,
            shadow: 1.5,
            alignment: Alignment::BottomCenter,
            lmargin: 96,
            rmargin: 96,
            vmargin: 65,
            encoding: 1,
            ..Default::default()
        };

        let mut ass_event = vec![];

        value.body.iter().for_each(|b| {
            ass_event.push(SSAEvent {
                style: "Default".into(),
                line_start: Time {
                    ms: (b.from * 1000.) as u32,
                    ..Default::default()
                },
                line_end: Time {
                    ms: (b.to * 1000.) as u32,
                    ..Default::default()
                },
                line_text: b.content.clone(),
                ..Default::default()
            })
        });

        Subtitle::SSA(Some(SSAFile {
            events: ass_event,
            styles: vec![ass_styles],
            info: ass_info,
            format: ".ass".into(),
        }))
    }
}

impl From<JsonSubtitle> for SSAFile {
    fn from(value: JsonSubtitle) -> Self {
        let mut ass_info = HashMap::new();
        ass_info.insert("Title".into(), "Bilibili Subtitle".into());
        ass_info.insert("ScriptType".into(), "v4.00+".into());
        ass_info.insert("WrapStyle".into(), "0".into());
        ass_info.insert("ScaledBorderAndShadow".into(), "yes".into());
        ass_info.insert("YCbCr Matrix".into(), "TV.601".into());
        ass_info.insert("PlayResX".into(), "1920".into());
        ass_info.insert("PlayResY".into(), "1080".into());

        let ass_styles = SSAStyle {
            name: "Default".into(),
            fontname: "Noto Sans".into(),
            fontsize: 100.,
            firstcolor: SSAColor(Color {
                a: 0,
                r: 255,
                g: 255,
                b: 255,
            }),
            secondcolor: SSAColor(Color {
                r: 0,
                g: 255,
                b: 255,
                a: 0,
            }),
            outlinecolor: SSAColor(Color {
                r: 8,
                g: 34,
                b: 0,
                a: 0,
            }),
            backgroundcolor: SSAColor(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 127,
            }),
            bold: false,
            italic: false,
            unerline: false,
            strikeout: false,
            scalex: 100.,
            scaley: 100.,
            spacing: 0.,
            angle: 0.,
            borderstyle: 1,
            outline: 5.,
            shadow: 1.5,
            alignment: Alignment::BottomCenter,
            lmargin: 96,
            rmargin: 96,
            vmargin: 65,
            encoding: 1,
            ..Default::default()
        };

        let mut ass_event = vec![];

        value.body.iter().for_each(|b| {
            ass_event.push(SSAEvent {
                style: "Default".into(),
                line_start: Time {
                    ms: (b.from * 1000.) as u32,
                    ..Default::default()
                },
                line_end: Time {
                    ms: (b.to * 1000.) as u32,
                    ..Default::default()
                },
                line_text: b.content.clone(),
                ..Default::default()
            })
        });

        SSAFile {
            events: ass_event,
            styles: vec![ass_styles],
            info: ass_info,
            format: ".ass".into(),
        }
    }
}

impl From<JsonSubtitle> for SRTFile {
    fn from(value: JsonSubtitle) -> Self {
        let mut srt_line = vec![];

        for (i, b) in value.body.iter().enumerate() {
            srt_line.push(SRTLine {
                line_number: (i + 1) as i32,
                line_text: b.content.clone(),
                line_start: Time {
                    ms: (b.from * 1000.) as u32,
                    ..Default::default()
                },
                line_end: Time {
                    ms: (b.to * 1000.) as u32,
                    ..Default::default()
                },
            });
        }

        SRTFile { lines: srt_line }
    }
}

impl From<JsonSubtitle> for VTTFile {
    fn from(value: JsonSubtitle) -> Self {
        let vtt_style = VTTStyle {
            name: Some("Default".into()),
            font_family: "Noto Sans".into(),
            font_size: "100".into(),
            color: ColorType::VTTColor(Color {
                r: 255,
                g: 255,
                b: 255,
                a: 0,
            }),
            background_color: ColorType::VTTColor(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 127,
            }),
            ..Default::default()
        };

        let mut vtt_lines = vec![];

        for (i, b) in value.body.iter().enumerate() {
            vtt_lines.push(VTTLine {
                line_number: i.to_string(),
                style: Some("Default".into()),
                line_start: Time {
                    ms: (b.from * 1000.) as u32,
                    ..Default::default()
                },
                line_end: Time {
                    ms: (b.to * 1000.) as u32,
                    ..Default::default()
                },
                position: None,
                line_text: b.content.clone(),
            })
        }

        VTTFile {
            styles: vec![vtt_style],
            lines: vtt_lines,
        }
    }
}

impl SubtitleFormat {
    /// Get the subtitle format of an episode.
    pub fn get_episode_subtitle_type(
        episode: &EpisodeMetadata<impl AsRef<Path>>,
        subtitle_language: &str,
    ) -> Result<Self> {
        match episode {
            EpisodeMetadata::Normal(e) => {
                Self::get_normal_episode_subtitle_type(e, subtitle_language)
            }
            EpisodeMetadata::Special(e) => {
                Self::get_special_episode_subtitle_type(e, subtitle_language)
            }
        }
    }

    /// Get the subtitle format of a normal episode.
    pub fn get_normal_episode_subtitle_type(
        episode: &NormalEpisodeMetadata<impl AsRef<Path>>,
        subtitle_language: &str,
    ) -> Result<Self> {
        let subtitle_path = episode
            .path
            .as_ref()
            .ok_or(format!(
                "Episode {} of {} doesn't have a path.",
                episode.episode, episode.title
            ))?
            .as_ref()
            .join(subtitle_language)
            .read_dir()?
            .next()
            .ok_or("Subtitle directory is empty")??
            .path();
        let extension = subtitle_path.extension().ok_or(format!(
            "Subtitle {} has no extension.",
            subtitle_path.display()
        ))?;

        match extension
            .to_str()
            .ok_or("OsStr doesn't yeild valid Unicode.")?
        {
            "json" => Ok(Self::Json),
            "ass" | "ssa" => Ok(Self::Ssa),
            "srt" => Ok(Self::Srt),
            "vtt" => Ok(Self::Vtt),
            _ => Err(Error::FromString(format!(
                "Invalid extension: {}",
                subtitle_path.display()
            ))),
        }
    }

    /// Get the subtitle format of a special episode.
    pub fn get_special_episode_subtitle_type(
        episode: &SpecialEpisodeMetadata<impl AsRef<Path>>,
        subtitle_language: &str,
    ) -> Result<Self> {
        let subtitle_path = episode
            .path
            .as_ref()
            .ok_or(format!(
                "{} of {} doesn't have a path.",
                episode.episode_name, episode.title
            ))?
            .as_ref()
            .join(subtitle_language)
            .read_dir()?
            .next()
            .ok_or("Subtitle directory is empty.")??
            .path();
        let extension = subtitle_path.extension().ok_or(format!(
            "Subtitle {} has no extension.",
            subtitle_path.display()
        ))?;

        match extension
            .to_str()
            .ok_or("OsStr doesn't yeild valid Unicode.")?
        {
            "json" => Ok(Self::Json),
            "ass" | "ssa" => Ok(Self::Ssa),
            "srt" => Ok(Self::Srt),
            "vtt" => Ok(Self::Vtt),
            _ => Err(Error::FromString(format!(
                "Invalid extension: {}",
                subtitle_path.display()
            ))),
        }
    }
}
