use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "arlon")]
#[command(about = "Show commits in HEAD that are not in the specified branch")]
pub struct Cli {
    #[arg(help = "Branch name to compare against")]
    pub branch: String,
    
    #[arg(short, long, value_enum, default_value = "simple", help = "Output format")]
    pub format: OutputFormat,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Simple,
    Json,
}
