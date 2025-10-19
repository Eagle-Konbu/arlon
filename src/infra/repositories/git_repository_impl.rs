use crate::domain::entities::{Commit, FileChange, FileChangeStatus};
use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::domain::value_objects::{BranchName, CommitHash, FilePath};
use git2::{Oid, Repository};
use std::collections::HashSet;

pub struct GitRepositoryImpl {
    repo: Repository,
}

impl GitRepositoryImpl {
    pub fn open(path: &str) -> Result<Self, GitRepositoryError> {
        let repo = Repository::open(path).map_err(|e| GitRepositoryError::GitOperationFailed {
            message: format!("Failed to open repository: {}", e),
        })?;

        Ok(Self { repo })
    }

    pub fn open_current_dir() -> Result<Self, GitRepositoryError> {
        Self::open(".")
    }
}

impl GitRepository for GitRepositoryImpl {
    fn get_commits_not_in_branch(
        &self,
        branch: &BranchName,
    ) -> Result<Vec<Commit>, GitRepositoryError> {
        let head = self
            .repo
            .head()
            .map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get HEAD: {}", e),
            })?;
        let head_commit =
            head.peel_to_commit()
                .map_err(|e| GitRepositoryError::GitOperationFailed {
                    message: format!("Failed to get HEAD commit: {}", e),
                })?;

        let branch_ref = self
            .repo
            .find_branch(branch.as_str(), git2::BranchType::Local)
            .map_err(|_| GitRepositoryError::BranchNotFound {
                branch: branch.to_string(),
            })?;
        let branch_commit = branch_ref.get().peel_to_commit().map_err(|e| {
            GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get branch commit: {}", e),
            }
        })?;

        let commits_in_branch = self.get_all_commits(branch_commit.id())?;

        let mut revwalk =
            self.repo
                .revwalk()
                .map_err(|e| GitRepositoryError::GitOperationFailed {
                    message: format!("Failed to create revwalk: {}", e),
                })?;
        revwalk
            .push(head_commit.id())
            .map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to push HEAD to revwalk: {}", e),
            })?;

        let mut commits = Vec::new();

        for oid in revwalk {
            let oid = oid.map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get commit OID: {}", e),
            })?;

            if !commits_in_branch.contains(&oid) {
                let commit = self.repo.find_commit(oid).map_err(|e| {
                    GitRepositoryError::GitOperationFailed {
                        message: format!("Failed to find commit: {}", e),
                    }
                })?;
                let author = commit.author();

                let hash = CommitHash::new(oid.to_string()).map_err(|e| {
                    GitRepositoryError::GitOperationFailed {
                        message: format!("Invalid commit hash: {}", e),
                    }
                })?;

                let domain_commit = Commit::new(
                    hash,
                    author.name().unwrap_or("").to_string(),
                    author.email().unwrap_or("").to_string(),
                    commit.time().seconds(),
                    commit.summary().unwrap_or("").to_string(),
                );

                commits.push(domain_commit);
            }
        }

        Ok(commits)
    }

    fn get_file_changes_between_branches(
        &self,
        branch: &BranchName,
    ) -> Result<Vec<FileChange>, GitRepositoryError> {
        let head = self
            .repo
            .head()
            .map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get HEAD: {}", e),
            })?;
        let head_commit =
            head.peel_to_commit()
                .map_err(|e| GitRepositoryError::GitOperationFailed {
                    message: format!("Failed to get HEAD commit: {}", e),
                })?;
        let head_tree = head_commit
            .tree()
            .map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get HEAD tree: {}", e),
            })?;

        let branch_ref = self
            .repo
            .find_branch(branch.as_str(), git2::BranchType::Local)
            .map_err(|_| GitRepositoryError::BranchNotFound {
                branch: branch.to_string(),
            })?;
        let branch_commit = branch_ref.get().peel_to_commit().map_err(|e| {
            GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get branch commit: {}", e),
            }
        })?;
        let branch_tree =
            branch_commit
                .tree()
                .map_err(|e| GitRepositoryError::GitOperationFailed {
                    message: format!("Failed to get branch tree: {}", e),
                })?;

        let mut diff_options = git2::DiffOptions::new();
        let diff = self
            .repo
            .diff_tree_to_tree(
                Some(&branch_tree),
                Some(&head_tree),
                Some(&mut diff_options),
            )
            .map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to create diff: {}", e),
            })?;

        let mut file_changes = Vec::new();

        diff.foreach(
            &mut |delta, _progress| {
                let status = match delta.status() {
                    git2::Delta::Unmodified => FileChangeStatus::Unmodified,
                    git2::Delta::Added => FileChangeStatus::Added,
                    git2::Delta::Deleted => FileChangeStatus::Deleted,
                    git2::Delta::Modified => FileChangeStatus::Modified,
                    git2::Delta::Renamed => FileChangeStatus::Renamed,
                    git2::Delta::Copied => FileChangeStatus::Copied,
                    git2::Delta::Ignored => FileChangeStatus::Ignored,
                    git2::Delta::Untracked => FileChangeStatus::Untracked,
                    git2::Delta::Typechange => FileChangeStatus::Typechange,
                    git2::Delta::Unreadable => FileChangeStatus::Unreadable,
                    git2::Delta::Conflicted => FileChangeStatus::Conflicted,
                };

                let path_str = if let Some(new_file) = delta.new_file().path() {
                    new_file.to_string_lossy().to_string()
                } else if let Some(old_file) = delta.old_file().path() {
                    old_file.to_string_lossy().to_string()
                } else {
                    return true; // Skip this delta
                };

                if let Ok(file_path) = FilePath::new(path_str) {
                    file_changes.push(FileChange::new(file_path, status));
                }

                true
            },
            None,
            None,
            None,
        )
        .map_err(|e| GitRepositoryError::GitOperationFailed {
            message: format!("Failed to process diff: {}", e),
        })?;

        Ok(file_changes)
    }
}

impl GitRepositoryImpl {
    fn get_all_commits(&self, start_oid: Oid) -> Result<HashSet<Oid>, GitRepositoryError> {
        let mut commits = HashSet::new();
        let mut revwalk =
            self.repo
                .revwalk()
                .map_err(|e| GitRepositoryError::GitOperationFailed {
                    message: format!("Failed to create revwalk: {}", e),
                })?;
        revwalk
            .push(start_oid)
            .map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to push OID to revwalk: {}", e),
            })?;

        for oid in revwalk {
            let oid = oid.map_err(|e| GitRepositoryError::GitOperationFailed {
                message: format!("Failed to get commit OID: {}", e),
            })?;
            commits.insert(oid);
        }

        Ok(commits)
    }
}
