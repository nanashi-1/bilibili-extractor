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

mod colorer;
mod compiler;
mod lister;

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub language: String,
    pub subtitle_type: SubtitleType,
    pub packager: Packager,
    pub input_path: String,
    pub is_parallel: bool,
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

        #[clap(long, short, help = "Compile episodes in parallel.")]
        parallel: bool,
    },
}

fn list(context: Context) -> Result<()> {
    let lister = Lister;
    let download_directory = DownloadFolder::new_from_path(context.input_path)?;

    lister.list_seasons(&download_directory.seasons);

    Ok(())
}

fn compile(context: Context) -> Result<()> {
    let download_directory = DownloadFolder::new_from_path(context.input_path.clone())?;
    let compiler = Compiler::new(context);

    compiler.compile_seasons(&download_directory.seasons)?;

    Ok(())
}

pub fn create_spinner(message: &str) -> Spinner {
    Spinner::new(spinners::Spinners::Dots, message.into())
}

fn main() {
    #[cfg(debug_assertions)]
    {
        println!("{}", "Debug Build!".color_as_warning());
        println!("More information will be printed.\n");
    }

    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::List { input } => {
            let context = Context {
                input_path: input,
                ..Default::default()
            };

            #[cfg(debug_assertions)]
            println!(
                "{} List Context: {:?}\n",
                "DEBUG:".color_as_warning(),
                context
            );

            let _ = list(context).inspect_err(|e| println!("{}", e.to_string().color_as_error()));
        }
        SubCommands::Compile {
            input,
            output,
            copy,
            language,
            use_hard_subtitle,
            parallel,
        } => {
            let context = Context {
                language,
                subtitle_type: match use_hard_subtitle {
                    true => SubtitleType::Hard,
                    false => SubtitleType::Soft,
                },
                packager: Packager {
                    output_path: output.into(),
                    config: PackagerConfig { copy },
                },
                input_path: input,
                is_parallel: parallel,
            };

            #[cfg(debug_assertions)]
            println!(
                "{} Compile Context: {:?}\n",
                "DEBUG:".color_as_warning(),
                context
            );

            let _ =
                compile(context).inspect_err(|e| println!("{}", e.to_string().color_as_error()));
        }
    }
}
