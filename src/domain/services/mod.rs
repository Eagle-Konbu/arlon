use crate::domain::entities::Commit;
use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::domain::value_objects::{BranchName, CommitHash};
use std::collections::HashSet;

/// コミット比較のビジネスロジックを担当するドメインサービス
pub struct CommitComparisonService;

impl CommitComparisonService {
    pub fn new() -> Self {
        Self
    }

    /// 指定されたブランチに含まれていないコミットを取得する
    /// 
    /// このメソッドは以下のビジネスロジックを実装します：
    /// 1. HEADから到達可能なすべてのコミットを取得
    /// 2. 指定されたブランチから到達可能なすべてのコミットを取得
    /// 3. 差集合を計算して、ブランチに含まれていないコミットを特定
    pub fn find_commits_not_in_branch<R: GitRepository>(
        &self,
        git_repository: &R,
        branch: &BranchName,
    ) -> Result<Vec<Commit>, GitRepositoryError> {
        let head_commits = git_repository.get_commits_from_head()?;
        let branch_commits = git_repository.get_commits_from_branch(branch)?;

        // ブランチのコミットハッシュをセットに変換して高速な検索を可能にする
        let branch_commit_hashes: HashSet<&CommitHash> = branch_commits
            .iter()
            .map(|commit| commit.hash())
            .collect();

        // HEADのコミットの中で、ブランチに含まれていないものを抽出
        let commits_not_in_branch: Vec<Commit> = head_commits
            .into_iter()
            .filter(|commit| !branch_commit_hashes.contains(commit.hash()))
            .collect();

        Ok(commits_not_in_branch)
    }
}

impl Default for CommitComparisonService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Commit;
    use crate::domain::repositories::{GitRepository, GitRepositoryError};
    use crate::domain::value_objects::{BranchName, CommitHash};
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        TestGitRepository {}

        impl GitRepository for TestGitRepository {
            fn get_commits_from_head(&self) -> Result<Vec<Commit>, GitRepositoryError>;
            fn get_commits_from_branch(&self, branch: &BranchName) -> Result<Vec<Commit>, GitRepositoryError>;
            fn get_file_changes_between_branches(&self, branch: &BranchName) -> Result<Vec<crate::domain::entities::FileChange>, GitRepositoryError>;
        }
    }

    #[test]
    fn test_find_commits_not_in_branch() {
        let mut mock_repo = MockTestGitRepository::new();
        let service = CommitComparisonService::new();
        let branch = BranchName::new("main".to_string()).unwrap();

        // テストデータの準備
        let commit1 = Commit::new(
            CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap(),
            "Alice".to_string(),
            "alice@example.com".to_string(),
            1234567890,
            "First commit".to_string(),
        );
        let commit2 = Commit::new(
            CommitHash::new("1234567890abcdef1234567890abcdef12345678".to_string()).unwrap(),
            "Bob".to_string(),
            "bob@example.com".to_string(),
            1234567891,
            "Second commit".to_string(),
        );
        let commit3 = Commit::new(
            CommitHash::new("fedcba0987654321fedcba0987654321fedcba09".to_string()).unwrap(),
            "Charlie".to_string(),
            "charlie@example.com".to_string(),
            1234567892,
            "Third commit".to_string(),
        );

        // HEADには3つのコミットがある
        let head_commits = vec![commit1.clone(), commit2.clone(), commit3.clone()];
        // ブランチには2つのコミットしかない
        let branch_commits = vec![commit1, commit2];

        mock_repo
            .expect_get_commits_from_head()
            .returning(move || Ok(head_commits.clone()));

        mock_repo
            .expect_get_commits_from_branch()
            .with(eq(branch.clone()))
            .returning(move |_| Ok(branch_commits.clone()));

        let result = service.find_commits_not_in_branch(&mock_repo, &branch).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].hash().as_str(), "fedcba0987654321fedcba0987654321fedcba09");
    }
}