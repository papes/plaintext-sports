use plaintext_sports::{
    types::{Average, GameId, InningsPitched, Score, TeamId},
    utils::{format_decimal_stat, format_score, parse_innings_pitched, parse_iso8601_date, truncate_string},
};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn test_team_id_from_str_never_panics(s in ".*") {
        let _ = TeamId::from_str(&s);
    }

    #[test]
    fn test_game_id_from_str_never_panics(s in ".*") {
        let _ = GameId::from_str(&s);
    }

    #[test]
    fn test_score_display_format(n in 0u32..1000) {
        let score = Score(n);
        prop_assert_eq!(score.to_string(), n.to_string());
    }

    #[test]
    fn test_average_valid_range(f in 0.0f32..=1.0f32) {
        let avg = Average::new(f);
        prop_assert!(avg.is_ok());
        let avg = avg.unwrap();
        let formatted = avg.format();
        prop_assert!(formatted.starts_with('.'));
        prop_assert_eq!(formatted.len(), 4);
    }

    #[test]
    fn test_average_invalid_range(f in prop_oneof![
        -1000.0f32..0.0f32,
        1.0f32..1000.0f32
    ]) {
        let avg = Average::new(f);
        prop_assert!(avg.is_err());
    }

    #[test]
    fn test_innings_pitched_valid_format(
        complete in 0u32..20,
        partial in 0u32..2
    ) {
        let ip_str = format!("{}.{}", complete, partial);
        let ip = InningsPitched::new(&ip_str);
        prop_assert!(ip.is_ok());
        let ip = ip.unwrap();
        prop_assert_eq!(ip.complete, complete);
        prop_assert_eq!(ip.partial, partial);
    }

    #[test]
    fn test_format_score_properties(opt_score in prop::option::of(0u32..1000)) {
        let formatted = format_score(opt_score);
        match opt_score {
            Some(n) => prop_assert_eq!(formatted, n.to_string()),
            None => prop_assert_eq!(formatted, "-"),
        }
    }

    #[test]
    fn test_format_decimal_stat_properties(opt_value in prop::option::of(0.0f32..1.0f32)) {
        let formatted = format_decimal_stat(opt_value);
        match opt_value {
            Some(_v) => {
                prop_assert!(formatted.starts_with('.'));
                prop_assert_eq!(formatted.len(), 4);
            },
            None => prop_assert_eq!(formatted, "---"),
        }
    }

    #[test]
    fn test_truncate_string_properties(
        s in ".*",
        max_len in 1usize..100
    ) {
        let truncated = truncate_string(&s, max_len);
        prop_assert!(truncated.len() <= max_len);
        if s.len() <= max_len {
            prop_assert_eq!(truncated, s);
        } else {
            prop_assert!(truncated.ends_with("..."));
        }
    }

    #[test]
    fn test_parse_innings_pitched_properties(
        complete in 0u32..20,
        partial in 0u32..2
    ) {
        let ip_str = format!("{}.{}", complete, partial);
        let result = parse_innings_pitched(&ip_str);
        let expected = complete as f32 + (partial as f32 / 3.0);
        prop_assert!((result - expected).abs() < 0.0001);
    }

    #[test]
    fn test_date_parsing_properties(
        year in 1900i32..2100,
        month in 1i32..13,
        day in 1i32..29,
        hour in 0i32..24,
        minute in 0i32..60,
        second in 0i32..60
    ) {
        let date_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            year, month, day, hour, minute, second
        );
        let result = parse_iso8601_date(&date_str);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_innings_pitched_roundtrip(
        complete in 0u32..20,
        partial in 0u32..3
    ) {
        let original = InningsPitched { complete, partial };
        if partial < 3 {
            let formatted = original.to_string();
            let parsed = InningsPitched::new(&formatted);
            prop_assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            prop_assert_eq!(parsed.complete, complete);
            prop_assert_eq!(parsed.partial, partial);
        } else {
            let ip_str = format!("{}.{}", complete, partial);
            prop_assert!(InningsPitched::new(&ip_str).is_err());
        }
    }

    #[test]
    fn test_average_roundtrip(avg in 0.0f32..=1.0f32) {
        let average = Average::new(avg).unwrap();
        let formatted = average.format();
        let parsed = formatted[1..].parse::<f32>().unwrap() / 1000.0;
        prop_assert!((avg - parsed).abs() < 0.001);
    }

    #[test]
    fn test_score_combinations(
        home in 0u32..30,
        away in 0u32..30,
        innings in 9u32..20
    ) {
        let home_score = Score(home);
        let away_score = Score(away);
        let ip = InningsPitched::new(&format!("{}.0", innings)).unwrap();
        
        // Test game scenarios
        prop_assert!(home_score.to_string().parse::<u32>().is_ok());
        prop_assert!(away_score.to_string().parse::<u32>().is_ok());
        prop_assert!(ip.as_float() >= 9.0);
    }

    #[test]
    fn test_string_truncation_unicode(
        s in "[\\p{L}\\p{N}\\p{P}\\p{S}]{0,100}",
        max_len in 1usize..50
    ) {
        let truncated = truncate_string(&s, max_len);
        prop_assert!(truncated.chars().count() <= max_len);
        if s.chars().count() <= max_len {
            prop_assert_eq!(truncated, s);
        } else {
            prop_assert!(truncated.ends_with("..."));
        }
    }
}

// Additional focused property tests for error cases
#[test]
fn test_innings_pitched_invalid_partial() {
    // Test that partial innings 3 or greater are rejected
    for partial in 3..10 {
        let ip_str = format!("6.{}", partial);
        assert!(InningsPitched::new(&ip_str).is_err());
    }
}

#[test]
fn test_average_edge_cases() {
    // Test exact boundaries
    assert!(Average::new(0.0).is_ok());
    assert!(Average::new(1.0).is_ok());
    assert!(Average::new(-0.001).is_err());
    assert!(Average::new(1.001).is_err());
}

#[test]
fn test_format_decimal_stat_edge_cases() {
    assert_eq!(format_decimal_stat(Some(0.0)), ".000");
    assert_eq!(format_decimal_stat(Some(1.0)), "1.000");
    assert_eq!(format_decimal_stat(Some(0.333333)), ".333");
}

// Add focused test cases for specific scenarios
#[test]
fn test_complex_game_scenarios() {
    // Test no-hitter game
    let no_hitter = InningsPitched::new("9.0").unwrap();
    assert_eq!(no_hitter.as_float(), 9.0);
    assert_eq!(Score(0).to_string(), "0");
    
    // Test extra innings game
    let extra_innings = InningsPitched::new("12.2").unwrap();
    assert!((extra_innings.as_float() - 12.666667).abs() < 0.0001);
    
    // Test suspended game
    let suspended = InningsPitched::new("5.1").unwrap();
    assert!((suspended.as_float() - 5.333333).abs() < 0.0001);
}

#[test]
fn test_batting_average_special_cases() {
    // Test perfect average (1.000)
    let perfect = Average::new(1.0).unwrap();
    assert_eq!(perfect.format(), "1.000");
    
    // Test zero average (.000)
    let zero = Average::new(0.0).unwrap();
    assert_eq!(zero.format(), ".000");
    
    // Test common averages
    let averages = [0.333, 0.275, 0.400, 0.198];
    for &avg in &averages {
        let batting_avg = Average::new(avg).unwrap();
        let formatted = batting_avg.format();
        assert_eq!(formatted.len(), 4);
        assert!(formatted.starts_with('.'));
    }
}

#[test]
fn test_date_edge_cases() {
    // Test year boundaries
    assert!(parse_iso8601_date("1900-01-01T00:00:00Z").is_ok());
    assert!(parse_iso8601_date("2099-12-31T23:59:59Z").is_ok());
    
    // Test invalid dates
    assert!(parse_iso8601_date("2024-02-30T00:00:00Z").is_err());
    assert!(parse_iso8601_date("2024-13-01T00:00:00Z").is_err());
    assert!(parse_iso8601_date("2024-01-01T24:00:00Z").is_err());
}

#[test]
fn test_string_manipulation_edge_cases() {
    // Test empty string
    assert_eq!(truncate_string("", 5), "");
    
    // Test exact length
    assert_eq!(truncate_string("12345", 5), "12345");
    
    // Test unicode characters
    assert_eq!(truncate_string("🏆⚾🎯", 2), "🏆...");
    
    // Test mixed ASCII and unicode
    assert_eq!(truncate_string("MLB⚾2024", 6), "MLB...");
} 