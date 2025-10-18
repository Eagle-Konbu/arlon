use arlon::cli::{Cli, Commands, OutputFormat};
use clap::Parser;

#[test]
fn test_cli_parse_commits_command() {
    let args = vec!["arlon", "commits", "main"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Commits { branch, format } => {
            assert_eq!(branch, "main");
            assert!(matches!(format, OutputFormat::Simple));
        }
        _ => panic!("Expected Commits command"),
    }
}

#[test]
fn test_cli_parse_commits_command_with_json_format() {
    let args = vec!["arlon", "commits", "develop", "--format", "json"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Commits { branch, format } => {
            assert_eq!(branch, "develop");
            assert!(matches!(format, OutputFormat::Json));
        }
        _ => panic!("Expected Commits command"),
    }
}

#[test]
fn test_cli_parse_files_command() {
    let args = vec!["arlon", "files", "feature-branch"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Files { branch, format } => {
            assert_eq!(branch, "feature-branch");
            assert!(matches!(format, OutputFormat::Simple));
        }
        _ => panic!("Expected Files command"),
    }
}

#[test]
fn test_cli_parse_files_command_with_short_format_flag() {
    let args = vec!["arlon", "files", "test-branch", "-f", "json"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Files { branch, format } => {
            assert_eq!(branch, "test-branch");
            assert!(matches!(format, OutputFormat::Json));
        }
        _ => panic!("Expected Files command"),
    }
}

#[test]
fn test_output_format_clone() {
    let format = OutputFormat::Json;
    let cloned_format = format.clone();
    
    assert!(matches!(cloned_format, OutputFormat::Json));
}

#[test]
fn test_cli_parse_invalid_command() {
    let args = vec!["arlon", "invalid"];
    let result = Cli::try_parse_from(args);
    
    assert!(result.is_err());
}

#[test]
fn test_cli_parse_missing_branch_argument() {
    let args = vec!["arlon", "commits"];
    let result = Cli::try_parse_from(args);
    
    assert!(result.is_err());
}

#[test]
fn test_cli_parse_invalid_format() {
    let args = vec!["arlon", "commits", "main", "--format", "invalid"];
    let result = Cli::try_parse_from(args);
    
    assert!(result.is_err());
}