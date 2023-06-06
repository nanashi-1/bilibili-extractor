#[test]
fn encode_episode_soft() {
    use bilibili_extractor::ffmpeg_controller::merge;

    merge(
        "tests/ffmpeg_encode_assets/video.m4s",
        "tests/ffmpeg_encode_assets/audio.m4s",
        "tests/ffmpeg_encode_assets/subtitle.ass",
        "tests/ffmpeg_encode_assets/episode_soft.mkv",
        false,
    )
    .unwrap();
}

#[test]
fn encode_episode_hard() {
    use bilibili_extractor::ffmpeg_controller::merge;

    merge(
        "tests/ffmpeg_encode_assets/video.m4s",
        "tests/ffmpeg_encode_assets/audio.m4s",
        "tests/ffmpeg_encode_assets/subtitle.ass",
        "tests/ffmpeg_encode_assets/episode_hard.mkv",
        true,
    )
    .unwrap();
}
