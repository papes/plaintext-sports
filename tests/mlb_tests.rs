use anyhow::Result;
use plaintext_sports::mlb::{Game, Team, GameState, GameTeams, Venue};

#[test]
fn test_game_state_parsing() -> Result<()> {
    assert_eq!(serde_json::from_str::<GameState>(r#""Final""#)?, GameState::Final);
    assert_eq!(serde_json::from_str::<GameState>(r#""Live""#)?, GameState::Live);
    assert_eq!(serde_json::from_str::<GameState>(r#""Scheduled""#)?, GameState::Scheduled);
    assert_eq!(serde_json::from_str::<GameState>(r#""Unknown""#)?, GameState::Unknown);
    Ok(())
}

#[test]
fn test_team_creation() -> Result<()> {
    let team = Team {
        id: 1,
        name: String::from("New York Yankees"),
        team_code: Some("nyy".to_string()),
        file_code: Some("nyy".to_string()),
        team_name: Some("Yankees".to_string()),
        location_name: Some("New York".to_string()),
        short_name: Some("Yankees".to_string()),
        abbreviation: Some("NYY".to_string()),
        franchise_name: None,
        club_name: None,
        first_year_of_play: None,
        active: None,
        venue: None,
        league: None,
        division: None,
    };
    
    assert_eq!(team.name, "New York Yankees");
    assert_eq!(team.team_code, Some("nyy".to_string()));
    assert_eq!(team.abbreviation, Some("NYY".to_string()));
    Ok(())
}

#[test]
fn test_game_creation() -> Result<()> {
    let home_team = Team {
        id: 2,
        name: String::from("Boston Red Sox"),
        team_code: Some("bos".to_string()),
        file_code: Some("bos".to_string()),
        team_name: Some("Red Sox".to_string()),
        location_name: Some("Boston".to_string()),
        short_name: Some("Red Sox".to_string()),
        abbreviation: Some("BOS".to_string()),
        franchise_name: None,
        club_name: None,
        first_year_of_play: None,
        active: None,
        venue: None,
        league: None,
        division: None,
    };
    
    let away_team = Team {
        id: 1,
        name: String::from("New York Yankees"),
        team_code: Some("nyy".to_string()),
        file_code: Some("nyy".to_string()),
        team_name: Some("Yankees".to_string()),
        location_name: Some("New York".to_string()),
        short_name: Some("Yankees".to_string()),
        abbreviation: Some("NYY".to_string()),
        franchise_name: None,
        club_name: None,
        first_year_of_play: None,
        active: None,
        venue: None,
        league: None,
        division: None,
    };
    
    let game = Game {
        game_pk: 12345,
        game_date: String::from("2024-03-14T19:05:00Z"),
        status: GameState::Final,
        teams: GameTeams {
            home: plaintext_sports::mlb::GameTeam {
                score: Some(6),
                team: home_team,
                is_winner: Some(true),
            },
            away: plaintext_sports::mlb::GameTeam {
                score: Some(5),
                team: away_team,
                is_winner: Some(false),
            },
        },
        venue: Some(Venue {
            id: 1,
            name: String::from("Fenway Park"),
        }),
    };
    
    assert_eq!(game.game_pk, 12345);
    assert_eq!(game.status, GameState::Final);
    assert_eq!(game.teams.home.score, Some(6));
    assert_eq!(game.teams.away.score, Some(5));
    Ok(())
} 