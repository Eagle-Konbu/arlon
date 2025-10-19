use crate::domain::entities::Commit;
use crate::domain::value_objects::CommitHash;
use std::collections::HashSet;

/// コミット比較のドメインロジックを担当するドメインサービス
/// 
/// ドメインサービスは、単一のエンティティや値オブジェクトに属さない
/// ビジネスロジックを実装するために使用されます。
pub struct CommitComparisonDomainService;

impl CommitComparisonDomainService {
    /// 指定されたブランチに含まれていないコミットを特定する
    /// 
    /// これは純粋なビジネスロジックであり、以下のルールに従います：
    /// 1. HEADから到達可能なすべてのコミットを基準とする
    /// 2. 指定されたブランチから到達可能なコミットを除外対象とする
    /// 3. 差集合を計算して、ブランチに含まれていないコミットを特定
    /// 
    /// # 引数
    /// * `head_commits` - HEADから到達可能なコミット一覧
    /// * `branch_commits` - 指定されたブランチから到達可能なコミット一覧
    /// 
    /// # 戻り値
    /// ブランチに含まれていないコミットの一覧
    pub fn find_commits_not_in_branch(
        head_commits: Vec<Commit>,
        branch_commits: Vec<Commit>,
    ) -> Vec<Commit> {
        // ブランチのコミットハッシュをセットに変換して高速な検索を可能にする
        let branch_commit_hashes: HashSet<&CommitHash> = branch_commits
            .iter()
            .map(|commit| commit.hash())
            .collect();

        // HEADのコミットの中で、ブランチに含まれていないものを抽出
        head_commits
            .into_iter()
            .filter(|commit| !branch_commit_hashes.contains(commit.hash()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Commit;
    use crate::domain::value_objects::CommitHash;

    #[test]
    fn test_find_commits_not_in_branch() {
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

        let result = CommitComparisonDomainService::find_commits_not_in_branch(
            head_commits,
            branch_commits,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].hash().as_str(), "fedcba0987654321fedcba0987654321fedcba09");
    }

    #[test]
    fn test_find_commits_all_commits_in_branch() {
        let commit1 = Commit::new(
            CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap(),
            "Alice".to_string(),
            "alice@example.com".to_string(),
            1234567890,
            "First commit".to_string(),
        );

        let head_commits = vec![commit1.clone()];
        let branch_commits = vec![commit1];

        let result = CommitComparisonDomainService::find_commits_not_in_branch(
            head_commits,
            branch_commits,
        );

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_commits_empty_branch() {
        let commit1 = Commit::new(
            CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap(),
            "Alice".to_string(),
            "alice@example.com".to_string(),
            1234567890,
            "First commit".to_string(),
        );

        let head_commits = vec![commit1.clone()];
        let branch_commits = vec![];

        let result = CommitComparisonDomainService::find_commits_not_in_branch(
            head_commits,
            branch_commits,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].hash().as_str(), "abcdef1234567890abcdef1234567890abcdef12");
    }
}