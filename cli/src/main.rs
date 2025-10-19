use cli::{Args, CommandController};
use clap::Parser;
use std::process;

mod cli;

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let controller = CommandController::new_with_current_dir()?;
    controller.execute(args.command)?;
    Ok(())
}