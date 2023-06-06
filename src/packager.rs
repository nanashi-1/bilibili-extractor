use std::{
    ffi::OsStr,
    fs::{copy, create_dir_all, rename},
    io::Error,
    path::Path,
};

use crate::metadata_reader::get_season_metadata;

/// Sets the behavior on how the season is going to be package
pub struct PackageConfig<P: AsRef<Path>> {
    pub episode_video_path: P,
    pub copy: bool,
}

impl Default for PackageConfig<&str> {
    fn default() -> Self {
        Self {
            episode_video_path: "episode.mkv",
            copy: true,
        }
    }
}

/// Packages a season into a format that JellyFin can read.
///
/// # Errors
///
/// This function returns an error if:
/// * [`get_season_metadata()`] returns an error.
/// * any directory in the path specified does not already exist and it could not be created otherwise.
/// * video is not a regular file or a symlink to a regular file.
/// * video does not exist.
/// * there is insufficient permissions.
///
/// # Examples
///
/// ```
/// use bilibili_extractor::packager::{package_season, PackageConfig};
///
/// package_season(
/// "tests/packaging_assets/s_36534",
/// "tests/packaging_assets",
/// PackageConfig::default(),
/// )
/// .unwrap();
/// ```
pub fn package_season<P: AsRef<OsStr> + AsRef<Path>>(
    season_path: P,
    output_directory: P,
    config: PackageConfig<P>,
) -> std::io::Result<()> {
    let season_metadata = get_season_metadata(season_path)?;

    let season_folder_path = &Path::new(&output_directory).join(season_metadata.title.trim());
    create_dir_all(season_folder_path)?;

    season_metadata
        .episodes
        .iter()
        .map(|e| {
            let video_path = e.path.join(&config.episode_video_path);

            if config.copy {
                copy(
                    video_path,
                    season_folder_path.join(format!(
                        "{} EP{:0>2}{}",
                        season_metadata.title,
                        e.index,
                        e.path
                            .as_path()
                            .extension()
                            .unwrap_or(OsStr::new(".mkv"))
                            .to_str()
                            .unwrap_or(".mkv")
                    )),
                )?;
            } else {
                rename(
                    video_path,
                    season_folder_path.join(format!(
                        "{} EP{:0>2}{}",
                        season_metadata.title,
                        e.index,
                        e.path
                            .as_path()
                            .extension()
                            .unwrap_or(OsStr::new(".mkv"))
                            .to_str()
                            .unwrap_or(".mkv")
                    )),
                )?;
            }

            Ok(())
        })
        .collect::<Result<Vec<()>, Error>>()?;

    Ok(())
}
