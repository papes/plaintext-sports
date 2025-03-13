use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: plaintext-sports [OPTIONS]"));
    Ok(())
}

#[test]
fn test_cli_version() -> Result<()> {
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("--version");
    cmd.assert().success();
    Ok(())
}

#[test]
fn test_cli_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Welcome to Plaintext Sports!"))
        .stdout(predicate::str::contains("Usage Examples:"))
        .stdout(predicate::str::contains("Get player stats:"))
        .stdout(predicate::str::contains("Get team stats:"))
        .stdout(predicate::str::contains("Get team schedule:"))
        .stdout(predicate::str::contains("Get game results:"));
    Ok(())
}

#[test]
fn test_cli_invalid_command() -> Result<()> {
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
    Ok(())
}

#[test]
fn test_cli_name_arg() -> Result<()> {
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("--name").arg("John");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, John!"))
        .stdout(predicate::str::contains("Usage Examples:"));
    Ok(())
} 