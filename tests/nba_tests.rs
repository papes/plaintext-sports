use anyhow::Result;
use plaintext_sports::nba::{Game, Team};
use plaintext_sports::mlb::GameState;
use serde_json::json;

#[test]
fn test_nba_game_status_parsing() -> Result<()> {
    let final_state: GameState = serde_json::from_value(json!("Final"))?;
    let live_state: GameState = serde_json::from_value(json!("Live"))?;
    let scheduled_state: GameState = serde_json::from_value(json!("Scheduled"))?;
    let unknown_state: GameState = serde_json::from_value(json!("Unknown"))?;
    let in_progress_state: GameState = serde_json::from_value(json!("In Progress"))?;

    assert_eq!(final_state, GameState::Final);
    assert_eq!(live_state, GameState::Live);
    assert_eq!(scheduled_state, GameState::Scheduled);
    assert_eq!(unknown_state, GameState::Unknown);
    assert_eq!(in_progress_state, GameState::Unknown); // "In Progress" is mapped to Unknown
    Ok(())
}

#[test]
fn test_nba_team_creation() -> Result<()> {
    let team = Team {
        id: 1,
        abbreviation: String::from("GSW"),
        city: String::from("Golden State"),
        conference: String::from("Western"),
        division: String::from("Pacific"),
        full_name: String::from("Golden State Warriors"),
        name: String::from("Warriors"),
    };
    
    assert_eq!(team.abbreviation, "GSW");
    assert_eq!(team.full_name, "Golden State Warriors");
    assert_eq!(team.conference, "Western");
    assert_eq!(team.division, "Pacific");
    Ok(())
}

#[test]
fn test_nba_game_creation() -> Result<()> {
    let home_team = Team {
        id: 2,
        abbreviation: String::from("LAL"),
        city: String::from("Los Angeles"),
        conference: String::from("Western"),
        division: String::from("Pacific"),
        full_name: String::from("Los Angeles Lakers"),
        name: String::from("Lakers"),
    };
    
    let visitor_team = Team {
        id: 1,
        abbreviation: String::from("GSW"),
        city: String::from("Golden State"),
        conference: String::from("Western"),
        division: String::from("Pacific"),
        full_name: String::from("Golden State Warriors"),
        name: String::from("Warriors"),
    };
    
    let game = Game {
        id: 12345,
        date: String::from("2024-03-14T22:30:00Z"),
        home_team,
        home_team_score: 115,
        period: 4,
        postseason: false,
        season: 2024,
        status: String::from("Final"),
        time: None,
        visitor_team,
        visitor_team_score: 110,
    };
    
    assert_eq!(game.id, 12345);
    assert_eq!(game.status, "Final");
    assert_eq!(game.home_team_score, 115);
    assert_eq!(game.visitor_team_score, 110);
    Ok(())
} 