use clap::Parser;
use git2::{Repository, Oid};
use std::collections::HashSet;
use std::process;
use chrono;

#[derive(Parser)]
#[command(name = "arlon")]
#[command(about = "Show commits in HEAD that are not in the specified branch")]
struct Cli {
    #[arg(help = "Branch name to compare against")]
    branch: String,
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(&cli.branch) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    
    let branch_ref = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let branch_commit = branch_ref.get().peel_to_commit()?;
    
    let commits_in_branch = get_all_commits(&repo, branch_commit.id())?;
    
    let mut revwalk = repo.revwalk()?;
    revwalk.push(head_commit.id())?;
    
    for oid in revwalk {
        let oid = oid?;
        if !commits_in_branch.contains(&oid) {
            let commit = repo.find_commit(oid)?;
            let summary = commit.summary().unwrap_or("<no message>");
            let author = commit.author();
            let time = commit.time();
            
            println!("commit {}", oid);
            println!("Author: {} <{}>", author.name().unwrap_or(""), author.email().unwrap_or(""));
            println!("Date:   {}", format_time(time));
            println!();
            println!("    {}", summary);
            println!();
        }
    }
    
    Ok(())
}

fn get_all_commits(repo: &Repository, start_oid: Oid) -> Result<HashSet<Oid>, git2::Error> {
    let mut commits = HashSet::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push(start_oid)?;
    
    for oid in revwalk {
        commits.insert(oid?);
    }
    
    Ok(commits)
}

fn format_time(time: git2::Time) -> String {
    let offset_minutes = time.offset_minutes();
    let offset_hours = offset_minutes / 60;
    let offset_sign = if offset_minutes >= 0 { '+' } else { '-' };
    
    let datetime = chrono::DateTime::from_timestamp(time.seconds(), 0)
        .unwrap_or_default();
    
    format!("{} {}{:02}{:02}", 
        datetime.format("%a %b %d %H:%M:%S %Y"),
        offset_sign,
        offset_hours.abs(),
        (offset_minutes.abs() % 60)
    )
}
