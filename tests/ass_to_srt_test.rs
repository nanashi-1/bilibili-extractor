use std::fs;

use bilibili_extractor::ass_to_srt::convert_ass_to_srt;

#[test]
fn convert_sample_ass_to_srt() {
    convert_ass_to_srt(
        "tests/ass_to_srt_assets/sample.ass",
        "tests/ass_to_srt_assets/output.srt",
    )
    .unwrap();

    assert_eq!(
        "9ba028f22afccd4afc8e1803827510ec",
        format!(
            "{:x}",
            md5::compute(fs::read("tests/ass_to_srt_assets/output.srt").unwrap())
        )
    )
}
