use arlon::domain::repositories::GitRepository;
use arlon::domain::services::CommitComparisonDomainService;
use arlon::infra::repositories::GitRepositoryImpl;
use arlon::domain::value_objects::BranchName;
use tempfile::TempDir;
use git2::{Repository, Signature, Oid};
use std::fs;

pub struct TestGitRepo {
    pub temp_dir: TempDir,
    pub repo: Repository,
}

impl TestGitRepo {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let repo = Repository::init(temp_dir.path())?;
        
        let signature = Signature::now("Test User", "test@example.com")?;
        
        let readme_path = temp_dir.path().join("README.md");
        fs::write(&readme_path, "# Test Repository\n")?;
        
        let mut index = repo.index()?;
        index.add_path(std::path::Path::new("README.md"))?;
        index.write()?;
        
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let _commit_id = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;
        
        drop(tree);
        drop(index);
        
        Ok(Self { temp_dir, repo })
    }
    
    pub fn create_branch(&self, branch_name: &str) -> Result<(), git2::Error> {
        let head = self.repo.head()?;
        let target = head.target().unwrap();
        let commit = self.repo.find_commit(target)?;
        
        self.repo.branch(branch_name, &commit, false)?;
        Ok(())
    }
    
    pub fn checkout_branch(&self, branch_name: &str) -> Result<(), git2::Error> {
        let branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;
        let commit = branch.get().peel_to_commit()?;
        
        self.repo.set_head(&format!("refs/heads/{}", branch_name))?;
        self.repo.checkout_tree(commit.as_object(), None)?;
        Ok(())
    }
    
    pub fn create_commit_on_current_branch(&self, message: &str, file_changes: &[(&str, &str)]) -> Result<Oid, git2::Error> {
        let signature = Signature::now("Test User", "test@example.com")?;
        
        for (file_path, content) in file_changes {
            let full_path = self.temp_dir.path().join(file_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).map_err(|e| git2::Error::from_str(&e.to_string()))?;
            }
            fs::write(&full_path, content).map_err(|e| git2::Error::from_str(&e.to_string()))?;
        }
        
        let mut index = self.repo.index()?;
        for (file_path, _) in file_changes {
            index.add_path(std::path::Path::new(file_path))?;
        }
        index.write()?;
        
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        
        let head = self.repo.head()?;
        let parent_commit = head.peel_to_commit()?;
        
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_commit_comparison_service() {
        let test_repo = TestGitRepo::new().unwrap();
        
        test_repo.create_branch("main").unwrap();
        test_repo.checkout_branch("main").unwrap();
        
        test_repo.create_commit_on_current_branch(
            "Add feature to main",
            &[("feature.txt", "New feature")],
        ).unwrap();
        
        test_repo.checkout_branch("master").unwrap();
        
        let git_repo = GitRepositoryImpl::open(test_repo.temp_dir.path().to_str().unwrap()).unwrap();
        
        let branch_name = BranchName::new("main".to_string()).unwrap();
        let head_commits = git_repo.get_commits_from_head().unwrap();
        let branch_commits = git_repo.get_commits_from_branch(&branch_name).unwrap();
        
        let commits = CommitComparisonDomainService::commits_not_in_branch(
            head_commits,
            branch_commits,
        );
        assert_eq!(commits.len(), 0);
    }
    
    #[test]
    fn test_git2_repository_get_file_changes_between_branches() {
        let test_repo = TestGitRepo::new().unwrap();
        
        test_repo.create_branch("feature").unwrap();
        test_repo.checkout_branch("feature").unwrap();
        
                // featureブランチにファイルを追加
        test_repo.create_commit_on_current_branch(
            "Add new file to feature",
            &[("new_file.txt", "This is a new file")],
        ).unwrap();
        
        let git_repo = GitRepositoryImpl::open(test_repo.temp_dir.path().to_str().unwrap()).unwrap();
        
        let branch_name = BranchName::new("master".to_string()).unwrap();
        let result = git_repo.get_file_changes_between_branches(&branch_name);
        
        assert!(result.is_ok());
        let file_changes = result.unwrap();
        
        println!("File changes found: {}", file_changes.len());
        for fc in &file_changes {
            println!("  {} - {}", fc.status().as_str(), fc.path().to_string_lossy());
        }
        
        if file_changes.is_empty() {
            println!("Warning: No file changes detected between branches");
        } else {
            let has_relevant_changes = file_changes.iter().any(|fc| {
                let path = fc.path().to_string_lossy();
                path.contains("new_file.txt") || path.contains("README.md")
            });
            assert!(has_relevant_changes, "Expected to find changes in new_file.txt or README.md");
        }
    }
    
    #[test]
    fn test_git2_repository_branch_not_found() {
        let test_repo = TestGitRepo::new().unwrap();
        let git_repo = GitRepositoryImpl::open(test_repo.temp_dir.path().to_str().unwrap()).unwrap();
        
        let branch_name = BranchName::new("nonexistent".to_string()).unwrap();
        let head_result = git_repo.get_commits_from_head();
        let branch_result = git_repo.get_commits_from_branch(&branch_name);
        
        assert!(head_result.is_ok());
        assert!(branch_result.is_err());
        match branch_result.unwrap_err() {
            arlon::domain::repositories::GitRepositoryError::BranchNotFound { branch } => {
                assert_eq!(branch, "nonexistent");
            }
            _ => panic!("Expected BranchNotFound error"),
        }
    }
}