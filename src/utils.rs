use chrono::{DateTime, NaiveDateTime, Utc};
use crate::error::{Result, SportError};
use crate::types::Average;
use std::cmp::Ordering;

/// Parses an ISO8601 date string into a DateTime<Utc>
pub fn parse_iso8601_date(date_str: &str) -> Result<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ")
        .map_err(|e| SportError::DateError(format!("Invalid date format: {}", e)))
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
}

/// Formats a score value for display, using "-" for None values
pub fn format_score(score: Option<u32>) -> String {
    score.map(|s| s.to_string()).unwrap_or_else(|| "-".to_string())
}

/// Formats a decimal statistic (like batting average) to three decimal places
pub fn format_decimal_stat(value: Option<f32>) -> String {
    value
        .map(|v| format!(".{:03}", (v * 1000.0).round() as u32))
        .unwrap_or_else(|| "---".to_string())
}

/// Truncates a string to a maximum length, adding "..." if truncated
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

/// Parses innings pitched string (e.g., "6.2") to float (6.666...)
pub fn parse_innings_pitched(ip: &str) -> f32 {
    let parts: Vec<&str> = ip.split('.').collect();
    match parts.len() {
        2 => {
            let innings = parts[0].parse::<f32>().unwrap_or(0.0);
            let fraction = parts[1].parse::<f32>().unwrap_or(0.0) / 3.0;
            innings + fraction
        }
        _ => ip.parse::<f32>().unwrap_or(0.0),
    }
}

/// Calculates earned run average (ERA) from earned runs and innings pitched
pub fn calculate_era(earned_runs: u32, innings_pitched: &str) -> Option<f32> {
    let ip = parse_innings_pitched(innings_pitched);
    if ip > 0.0 {
        let era = (earned_runs as f32 * 9.0) / ip;
        // Round to 1 decimal place
        Some((era * 10.0).round() / 10.0)
    } else {
        None
    }
}

/// Calculates batting average from hits and at-bats
pub fn calculate_average(hits: u32, at_bats: u32) -> Option<Average> {
    if at_bats > 0 {
        Average::new(hits as f32 / at_bats as f32).ok()
    } else {
        None
    }
}

/// Formats a win-loss record (e.g., "42-34")
pub fn format_record(wins: u32, losses: u32) -> String {
    format!("{}-{}", wins, losses)
}

/// Parses a win-loss record string into its components
pub fn parse_record(record: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = record.split('-').collect();
    if parts.len() != 2 {
        return Err(SportError::FetchError(format!(
            "Invalid record format: {}",
            record
        )));
    }

    let wins = parts[0].parse().map_err(|_| {
        SportError::FetchError(format!("Invalid wins in record: {}", record))
    })?;
    let losses = parts[1].parse().map_err(|_| {
        SportError::FetchError(format!("Invalid losses in record: {}", record))
    })?;

    Ok((wins, losses))
}

/// Compares two optional scores and returns the winning team index (0 for away, 1 for home)
pub fn determine_winner(away_score: Option<u32>, home_score: Option<u32>) -> Option<usize> {
    match (away_score, home_score) {
        (Some(away), Some(home)) => match away.cmp(&home) {
            Ordering::Greater => Some(0),
            Ordering::Less => Some(1),
            Ordering::Equal => None,
        },
        _ => None,
    }
}

/// Formats a player's name in "LAST, First" format
pub fn format_player_name(first: &str, last: &str) -> String {
    format!("{}, {}", last.to_uppercase(), first)
}

/// Formats a game time in local timezone
pub fn format_game_time(date_str: &str) -> Result<String> {
    let dt = parse_iso8601_date(date_str)?;
    Ok(dt.format("%I:%M %p").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iso8601_date() {
        let result = parse_iso8601_date("2024-03-28T19:05:00Z");
        assert!(result.is_ok());
        
        let error = parse_iso8601_date("invalid-date");
        assert!(error.is_err());
    }

    #[test]
    fn test_format_score() {
        assert_eq!(format_score(Some(5)), "5");
        assert_eq!(format_score(None), "-");
    }

    #[test]
    fn test_format_decimal_stat() {
        assert_eq!(format_decimal_stat(Some(0.333)), ".333");
        assert_eq!(format_decimal_stat(None), "---");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("very long string", 10), "very lo...");
    }

    #[test]
    fn test_parse_innings_pitched() {
        assert_eq!(parse_innings_pitched("6.0"), 6.0);
        assert_eq!(parse_innings_pitched("6.2"), 6.6666665);
        assert_eq!(parse_innings_pitched("invalid"), 0.0);
    }

    #[test]
    fn test_calculate_era() {
        assert_eq!(calculate_era(27, "81.0"), Some(3.0));
        assert_eq!(calculate_era(3, "9.0"), Some(3.0));
        assert_eq!(calculate_era(2, "6.2"), Some(2.7));
        assert_eq!(calculate_era(1, "0.0"), None);
    }

    #[test]
    fn test_calculate_average() {
        assert_eq!(
            calculate_average(100, 300).map(|a| a.format()),
            Some(".333".to_string())
        );
        assert_eq!(
            calculate_average(0, 10).map(|a| a.format()),
            Some(".000".to_string())
        );
        assert_eq!(calculate_average(5, 0), None);
    }

    #[test]
    fn test_format_record() {
        assert_eq!(format_record(42, 34), "42-34");
        assert_eq!(format_record(0, 0), "0-0");
    }

    #[test]
    fn test_parse_record() {
        assert_eq!(parse_record("42-34"), Ok((42, 34)));
        assert!(parse_record("42-").is_err());
        assert!(parse_record("invalid").is_err());
    }

    #[test]
    fn test_determine_winner() {
        assert_eq!(determine_winner(Some(5), Some(3)), Some(0));
        assert_eq!(determine_winner(Some(3), Some(5)), Some(1));
        assert_eq!(determine_winner(Some(4), Some(4)), None);
        assert_eq!(determine_winner(None, Some(5)), None);
        assert_eq!(determine_winner(Some(5), None), None);
    }

    #[test]
    fn test_format_player_name() {
        assert_eq!(format_player_name("Mike", "Trout"), "TROUT, Mike");
        assert_eq!(format_player_name("", ""), ", ");
    }

    #[test]
    fn test_format_game_time() {
        assert!(format_game_time("2024-03-14T19:05:00Z").is_ok());
        assert!(format_game_time("invalid").is_err());
    }
} 