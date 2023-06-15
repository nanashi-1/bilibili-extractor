use rsubs_lib::ssa;

/// This function will convert ASS files into SRT.
///
/// # Errors
///
/// This function will return an error if the ass file is invalid or have errors writing a file
///
/// # Example
///
/// ```
/// use std::fs;
/// use bilibili_extractor::ass_to_srt::convert_ass_to_srt;
///
/// convert_ass_to_srt(
///     "tests/ass_to_srt_assets/sample.ass",
///     "tests/ass_to_srt_assets/output.srt",
/// )
/// .unwrap();
///
/// assert_eq!(
///     "9ba028f22afccd4afc8e1803827510ec",
///     format!(
///         "{:x}",
///         md5::compute(fs::read("tests/ass_to_srt_assets/output.srt").unwrap())
///     )
/// )
/// ```
pub fn convert_ass_to_srt(ass_file_path: &str, srt_file_path: &str) -> std::io::Result<()> {
    ssa::parse(ass_file_path.to_owned())?
        .to_srt()
        .to_file(srt_file_path)?;

    Ok(())
}
