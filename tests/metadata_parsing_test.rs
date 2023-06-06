use bilibili_extractor::metadata_reader::{get_episode_metadata, get_season_metadata};

#[test]
fn read_episode_metadata() {
    let episode_metadata =
        get_episode_metadata("tests/metadata_parsing_assets/s_36534/377500").unwrap();
    assert_eq!(
        "EP 1 - Say what you want; famous games are usually fun",
        episode_metadata.title
    );
    assert_eq!("1", episode_metadata.index);
}

#[test]
fn read_season_metadata() {
    let season_metadata = get_season_metadata("tests/metadata_parsing_assets/s_36534").unwrap();

    assert_eq!("Bottom-tier Character Tomozaki ", season_metadata.title);
    assert_eq!(14, season_metadata.episodes.len());
}
