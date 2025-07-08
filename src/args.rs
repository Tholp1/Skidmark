use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProgramArgs {
    #[clap(subcommand)]
    command: Command,

    #[arg(
        long,
        default_value_t = false,
        help = "Console output will not be colored"
    )]
    no_color: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init {
        #[arg(short, long, default_value = ".")]
        folder: String,
    },

    Build {
        #[arg(
            short = 'f',
            long,
            default_value = "",
            value_name = "PROJECT FILE",
            help = "Override skidmark.toml"
        )]
        project_file: String,
    },

    Clean {
        #[arg(
            short = 'f',
            long,
            default_value = "",
            value_name = "PROJECT FILE",
            help = "Override skidmark.toml"
        )]
        project_file: String,
    },
}
