use crate::domain::entities::Commit;
use crate::domain::value_objects::CommitHash;
use std::collections::HashSet;

pub struct CommitComparisonDomainService;

impl CommitComparisonDomainService {
    pub fn commits_not_in_branch(
        head_commits: Vec<Commit>,
        branch_commits: Vec<Commit>,
    ) -> Vec<Commit> {
        let branch_commit_hashes: HashSet<&CommitHash> =
            branch_commits.iter().map(|commit| commit.hash()).collect();

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
    fn test_commits_not_in_branch() {
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

        let head_commits = vec![commit1.clone(), commit2.clone(), commit3.clone()];
        let branch_commits = vec![commit1, commit2];

        let result =
            CommitComparisonDomainService::commits_not_in_branch(head_commits, branch_commits);

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].hash().as_str(),
            "fedcba0987654321fedcba0987654321fedcba09"
        );
    }

    #[test]
    fn test_all_commits_in_branch() {
        let commit1 = Commit::new(
            CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap(),
            "Alice".to_string(),
            "alice@example.com".to_string(),
            1234567890,
            "First commit".to_string(),
        );

        let head_commits = vec![commit1.clone()];
        let branch_commits = vec![commit1];

        let result =
            CommitComparisonDomainService::commits_not_in_branch(head_commits, branch_commits);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_empty_branch() {
        let commit1 = Commit::new(
            CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap(),
            "Alice".to_string(),
            "alice@example.com".to_string(),
            1234567890,
            "First commit".to_string(),
        );

        let head_commits = vec![commit1.clone()];
        let branch_commits = vec![];

        let result =
            CommitComparisonDomainService::commits_not_in_branch(head_commits, branch_commits);

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].hash().as_str(),
            "abcdef1234567890abcdef1234567890abcdef12"
        );
    }
}
