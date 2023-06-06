use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::Path,
};

use serde_derive::Deserialize;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
struct BilibiliJSONSubtitleFile {
    body: Vec<BilibiliJSONBodyPart>,
}

#[derive(Deserialize, Debug)]
struct BilibiliJSONBodyPart {
    from: f32,
    to: f32,
    content: String,
}

/// This function converts JSON files from Bilibili's downloads into a SubStation Alpha Subtitles(.ASS) file.
///
/// # Errors
///
/// This function returns an error if:
///
/// * the file is not a valid Bilibili JSON subtitle file.
/// * the full directory path does not exist.
/// * not all bytes could be written due to I/O errors or EOF being reached.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use bilibili_extractor::json_to_ass::convert_json_to_ass;
///
/// convert_json_to_ass(
///     "tests/json_to_ass_assets/subtitle_sample.json",
///     "tests/json_to_ass_assets/subtitle_sample_out.ass",
/// )
/// .unwrap();
///
/// assert_eq!(
///     "a69099a6bdb92e461cc0cbf5f417481e",
///     format!(
///         "{:x}",
///         md5::compute(fs::read("tests/json_to_ass_assets/subtitle_sample_out.ass").unwrap())
///     )
/// )
/// ```
pub fn convert_json_to_ass<P: AsRef<Path>>(json_path: P, ass_path: P) -> std::io::Result<()> {
    let json_data = {
        let string = read_to_string(json_path)?;

        from_str::<BilibiliJSONSubtitleFile>(&string)?.body
    };

    let script_info = include_str!("../assets/script_info.p.ass");
    let styles_info = include_str!("../assets/styles_info.p.ass");
    let events_info = include_str!("../assets/events_info.p.ass");

    let mut ass_file = BufWriter::new(File::create(ass_path)?);

    ass_file.write_all(script_info.as_bytes())?;
    ass_file.write_all(styles_info.as_bytes())?;
    ass_file.write_all(events_info.as_bytes())?;

    json_data.iter().try_for_each(|v| {
        ass_file.write_all(
            format!(
                "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
                second_to_ass_time_format(v.from),
                second_to_ass_time_format(v.to),
                v.content.replace('\n', "\\N")
            )
            .as_bytes(),
        )
    })?;

    ass_file.flush()?;

    Ok(())
}

fn second_to_ass_time_format(seconds: f32) -> String {
    let centisecond = (seconds.fract() * 100.0).round() as i32;
    let seconds = seconds.trunc() as i32;
    let second = seconds % 60;
    let minute = (seconds / 60) % 60;
    let hour = (seconds / 60) / 60;

    format!("{hour}:{minute:0>2}:{second:0>2}.{centisecond:0>2}")
}
