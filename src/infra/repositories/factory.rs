use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::infra::repositories::GitRepositoryImpl;

/// GitRepository実装のファクトリ
pub struct GitRepositoryFactory;

impl GitRepositoryFactory {
    /// 指定されたパスでGitRepositoryを開く
    pub fn open(path: &str) -> Result<Box<dyn GitRepository>, GitRepositoryError> {
        let repo = GitRepositoryImpl::open(path)?;
        Ok(Box::new(repo))
    }

    /// カレントディレクトリでGitRepositoryを開く
    pub fn open_current_dir() -> Result<Box<dyn GitRepository>, GitRepositoryError> {
        let repo = GitRepositoryImpl::open_current_dir()?;
        Ok(Box::new(repo))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use git2::{Repository, Signature};

    #[test]
    fn test_factory_open_repository() {
        // テスト用のGitリポジトリを作成
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        
        // 初期コミットを作成
        let signature = Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        ).unwrap();

        // ファクトリでリポジトリを開く
        let git_repo = GitRepositoryFactory::open(temp_dir.path().to_str().unwrap());
        assert!(git_repo.is_ok());
    }
}