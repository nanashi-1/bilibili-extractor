use crate::colorer::Colorer;
use bilibili_extractor_lib::{
    error::Result,
    metadata::DownloadFolder,
    packager::{Packager, PackagerConfig},
    subtitle::SubtitleType,
};
use clap::{Parser, Subcommand};
use compiler::Compiler;
use lister::Lister;
use spinners::Spinner;
use std::path::Path;

mod colorer;
mod compiler;
mod lister;

#[derive(Clone, Copy, Default)]
pub struct Context<'a, P: AsRef<Path>> {
    pub language: &'a str,
    pub subtitle_type: SubtitleType,
    pub packager: Packager<P>,
    pub input_path: &'a str,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    #[command(about = "List all seasons in the input directory.")]
    List {
        #[arg(required = true, help = "The input directory.")]
        input: String,
    },

    #[command(about = "Compile all seasons in the input directory to the output directory.")]
    Compile {
        #[arg(help = "The input directory.")]
        input: String,

        #[arg(help = "The output direcctory.")]
        output: String,

        #[clap(long, short, help = "Copy combined files.")]
        copy: bool,

        #[clap(long, short, help = "Set language for the subtitle.", default_value_t = String::from("en"))]
        language: String,

        #[clap(long, help = "Set language for the subtitle.")]
        use_hard_subtitle: bool,
    },
}

fn list(context: Context<&str>) -> Result<()> {
    let lister = Lister;
    let download_directory = DownloadFolder::new_from_path(context.input_path)?;

    lister.list_seasons(&download_directory.seasons);

    Ok(())
}

fn compile(context: Context<&str>) -> Result<()> {
    let compiler = Compiler::new(context);
    let download_directory = DownloadFolder::new_from_path(context.input_path)?;

    compiler.compile_seasons(&download_directory.seasons)?;

    Ok(())
}

pub fn create_spinner(message: &str) -> Spinner {
    Spinner::new(spinners::Spinners::Dots, message.into())
}

fn main() {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::List { input } => {
            let context = Context {
                input_path: &input,
                ..Default::default()
            };

            if let Err(e) = list(context) {
                println!("{}", e.to_string().color_as_error())
            }
        }
        SubCommands::Compile {
            input,
            output,
            copy,
            language,
            use_hard_subtitle,
        } => {
            let context: Context<'_, &str> = Context {
                language: &language,
                subtitle_type: match use_hard_subtitle {
                    true => SubtitleType::Hard,
                    false => SubtitleType::Soft,
                },
                packager: Packager {
                    output_path: &output,
                    config: PackagerConfig { copy },
                },
                input_path: &input,
            };

            if let Err(e) = compile(context) {
                println!("{}", e.to_string().color_as_error())
            }
        }
    }
}
