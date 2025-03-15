use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;

#[test]
fn test_mlb_today_workflow() -> Result<()> {
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("--todays-games").arg("--leagues").arg("MLB");
    cmd.timeout(Duration::from_secs(10));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Today's MLB Games"));
    Ok(())
}

#[test]
fn test_nba_today_workflow() -> Result<()> {
    // Skip this test if NBA_API_KEY is not set
    if std::env::var("NBA_API_KEY").is_err() {
        println!("Skipping NBA test because NBA_API_KEY is not set");
        return Ok(());
    }
    
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("--todays-games").arg("--leagues").arg("NBA");
    cmd.timeout(Duration::from_secs(10));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("NBA Games"));
    Ok(())
}

#[test]
fn test_error_handling() -> Result<()> {
    // Test invalid team ID
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("mlb").arg("team").arg("--id").arg("999999");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Error fetching MLB team data"))
        .stdout(predicate::str::contains("Not Found"));

    Ok(())
}

#[test]
fn test_config_loading() -> Result<()> {
    // Test that the help command shows the expected sections
    let mut cmd = Command::cargo_bin("plaintext-sports")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("Options:"));
    Ok(())
} 