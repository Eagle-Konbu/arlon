mod cli;
mod git;
mod output;

use clap::Parser;
use cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(&cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let commits = git::get_commits_not_in_branch(&cli.branch)?;
    output::print_commits(&commits, &cli.format)?;
    Ok(())
}
