use std::fs;

#[test]
fn convert_sample_json_to_ass() {
    use bilibili_extractor::json_to_ass::convert_json_to_ass;

    convert_json_to_ass(
        "tests/json_to_ass_assets/subtitle_sample.json",
        "tests/json_to_ass_assets/subtitle_sample_out.ass",
    )
    .unwrap();

    assert_eq!(
        "a69099a6bdb92e461cc0cbf5f417481e",
        format!(
            "{:x}",
            md5::compute(fs::read("tests/json_to_ass_assets/subtitle_sample_out.ass").unwrap())
        )
    )
}
