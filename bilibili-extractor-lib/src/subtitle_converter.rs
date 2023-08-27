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
use std::{collections::HashMap, fs, path::PathBuf};

use crate::error::Error;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct JsonSubtitleBody {
    pub from: f32,
    pub to: f32,
    pub content: String,
}

impl JsonSubtitle {
    pub fn to_subtitle(self) -> Subtitle {
        self.into()
    }

    pub fn to_ssa(self) -> SSAFile {
        self.into()
    }

    pub fn to_srt(self) -> SRTFile {
        self.into()
    }

    pub fn to_vtt(self) -> VTTFile {
        self.into()
    }
}

impl TryFrom<PathBuf> for JsonSubtitle {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let json_string = fs::read_to_string(value)?;

        Ok(serde_json::from_str(&json_string)?)
    }
}

impl Into<Subtitle> for JsonSubtitle {
    fn into(self) -> Subtitle {
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

        self.body.iter().for_each(|b| {
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

impl Into<SSAFile> for JsonSubtitle {
    fn into(self) -> SSAFile {
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

        self.body.iter().for_each(|b| {
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

impl Into<SRTFile> for JsonSubtitle {
    fn into(self) -> SRTFile {
        let mut srt_line = vec![];

        for (i, b) in self.body.iter().enumerate() {
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

impl Into<VTTFile> for JsonSubtitle {
    fn into(self) -> VTTFile {
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

        for (i, b) in self.body.iter().enumerate() {
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
