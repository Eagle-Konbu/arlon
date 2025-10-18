use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "arlon")]
#[command(about = "Compare branches and files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show commits in HEAD that are not in the specified branch
    Commits {
        #[arg(help = "Branch name to compare against")]
        branch: String,
        
        #[arg(short, long, value_enum, default_value = "simple", help = "Output format")]
        format: OutputFormat,
    },
    /// Show files that differ between branches
    Files {
        #[arg(help = "Branch name to compare against")]
        branch: String,
        
        #[arg(short, long, value_enum, default_value = "simple", help = "Output format")]
        format: OutputFormat,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Simple,
    Json,
}
