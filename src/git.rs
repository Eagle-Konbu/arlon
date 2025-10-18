use git2::{Repository, Oid};
use std::collections::HashSet;

pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub timestamp: i64,
    pub message: String,
}

pub fn get_commits_not_in_branch(branch_name: &str) -> Result<Vec<CommitInfo>, Box<dyn std::error::Error>> {
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
                timestamp: commit.time().seconds(),
                message: commit.summary().unwrap_or("").to_string(),
            };
            
            commit_infos.push(commit_info);
        }
    }
    
    Ok(commit_infos)
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
