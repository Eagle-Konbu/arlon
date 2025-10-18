use clap::{Parser, ValueEnum};
use git2::{Repository, Oid};
use serde::Serialize;
use std::collections::HashSet;
use std::process;

#[derive(Parser)]
#[command(name = "arlon")]
#[command(about = "Show commits in HEAD that are not in the specified branch")]
struct Cli {
    #[arg(help = "Branch name to compare against")]
    branch: String,
    
    #[arg(short, long, value_enum, default_value = "simple", help = "Output format")]
    format: OutputFormat,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Simple,
    Json,
}

#[derive(Serialize)]
struct CommitInfo {
    hash: String,
    author: String,
    email: String,
    date: String,
    message: String,
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(&cli.branch, &cli.format) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(branch_name: &str, format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    
    let branch_ref = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let branch_commit = branch_ref.get().peel_to_commit()?;
    
    let commits_in_branch = get_all_commits(&repo, branch_commit.id())?;
    
    let mut revwalk = repo.revwalk()?;
    revwalk.push(head_commit.id())?;
    
    let mut commit_infos = Vec::new();
    
    for oid in revwalk {
        let oid = oid?;
        if !commits_in_branch.contains(&oid) {
            let commit = repo.find_commit(oid)?;
            let author = commit.author();
            
            let commit_info = CommitInfo {
                hash: oid.to_string(),
                author: author.name().unwrap_or("").to_string(),
                email: author.email().unwrap_or("").to_string(),
                date: format_timestamp(commit.time().seconds()),
                message: commit.summary().unwrap_or("").to_string(),
            };
            
            commit_infos.push(commit_info);
        }
    }
    
    match format {
        OutputFormat::Simple => {
            for info in commit_infos {
                println!("{} {} {}", info.hash, info.date, info.message);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&commit_infos)?;
            println!("{}", json);
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

fn format_timestamp(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    let datetime = DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| DateTime::<Utc>::default());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
