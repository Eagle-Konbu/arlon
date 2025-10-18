use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "arlon")]
#[command(about = "Compare branches and files")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Commits {
        #[arg(help = "Branch name to compare against")]
        branch: String,

        #[arg(
            short,
            long,
            value_enum,
            default_value = "simple",
            help = "Output format"
        )]
        format: OutputFormat,
    },
    Files {
        #[arg(help = "Branch name to compare against")]
        branch: String,

        #[arg(
            short,
            long,
            value_enum,
            default_value = "simple",
            help = "Output format"
        )]
        format: OutputFormat,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Simple,
    Json,
}
