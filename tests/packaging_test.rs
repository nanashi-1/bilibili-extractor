use bilibili_extractor::packager::{package_season, PackageConfig};

#[test]
fn package_season_copy() {
    package_season(
        "tests/packaging_assets/s_36534",
        "tests/packaging_assets",
        PackageConfig::default(),
    )
    .unwrap();
}
