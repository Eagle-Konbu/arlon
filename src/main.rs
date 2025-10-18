mod cli;
mod git;
mod output;

use clap::Parser;
use cli::{Cli, Commands};
use std::process;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(&cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Commits { branch, format } => {
            let commits = git::get_commits_not_in_branch(branch)?;
            output::print_commits(&commits, format)?;
        }
        Commands::Files { branch, format } => {
            let files = git::get_file_diff_between_branches(branch)?;
            output::print_files(&files, format)?;
        }
    }
    Ok(())
}
