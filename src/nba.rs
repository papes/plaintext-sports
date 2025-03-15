use anyhow::{anyhow, Result};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::{Local, NaiveDate};
use std::sync::OnceLock;

// Base URL for the balldontlie API
fn get_nba_api_base_url() -> String {
    crate::config::get_config().nba_api_base_url.clone()
}

// API key for the balldontlie API
fn get_nba_api_key() -> String {
    crate::config::get_config().nba_api_key.clone()
}

/// NBA API client
pub struct NbaApi {
    client: Client,
}

impl NbaApi {
    /// Create a new NBA API client
    pub fn new() -> Self {
        let client = create_client().expect("Failed to create HTTP client");
        Self { client }
    }
    
    /// Get team information by ID
    pub async fn get_team(&self, team_id: u32) -> Result<Team> {
        let url = format!("{}/teams/{}", get_nba_api_base_url(), team_id);
        let response = self.client.get(&url)
            .header(header::AUTHORIZATION, get_nba_api_key())
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get team: HTTP {}", response.status()));
        }
        
        let team_response: SingleResponse<Team> = response.json().await?;
        Ok(team_response.data)
    }
    
    /// Get player information by ID
    pub async fn get_player(&self, player_id: u32) -> Result<Player> {
        let url = format!("{}/players/{}", get_nba_api_base_url(), player_id);
        let response = self.client.get(&url)
            .header(header::AUTHORIZATION, get_nba_api_key())
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get player: HTTP {}", response.status()));
        }
        
        let player_response: SingleResponse<Player> = response.json().await?;
        Ok(player_response.data)
    }
    
    /// Get all NBA games for today
    pub async fn get_todays_games(&self) -> Result<Vec<Game>> {
        let today = Local::now().date_naive();
        self.get_games_by_date(today).await
    }
    
    /// Get all NBA games from yesterday
    pub async fn get_yesterdays_games(&self) -> Result<Vec<Game>> {
        let yesterday = Local::now().date_naive().pred_opt().ok_or_else(|| anyhow!("Failed to calculate yesterday's date"))?;
        self.get_games_by_date(yesterday).await
    }
    
    /// Get games by date
    pub async fn get_games_by_date(&self, date: NaiveDate) -> Result<Vec<Game>> {
        let date_str = date.format("%Y-%m-%d").to_string();
        let url = format!("{}/games?dates[]={}", get_nba_api_base_url(), date_str);
        
        let response = self.client.get(&url)
            .header(header::AUTHORIZATION, get_nba_api_key())
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get games: HTTP {}", response.status()));
        }
        
        let games_response: PaginatedResponse<Game> = response.json().await?;
        Ok(games_response.data)
    }
    
    /// Get team games
    pub async fn get_team_games(&self, team_id: u32, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<Game>> {
        let mut url = format!("{}/games?team_ids[]={}", get_nba_api_base_url(), team_id);
        
        if let Some(start) = start_date {
            url.push_str(&format!("&start_date={}", start));
        }
        
        if let Some(end) = end_date {
            url.push_str(&format!("&end_date={}", end));
        }
        
        let response = self.client.get(&url)
            .header(header::AUTHORIZATION, get_nba_api_key())
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get team games: HTTP {}", response.status()));
        }
        
        let games_response: PaginatedResponse<Game> = response.json().await?;
        Ok(games_response.data)
    }
    
    /// Get player stats for a game
    pub async fn get_game_player_stats(&self, game_id: u32) -> Result<Vec<PlayerStats>> {
        let url = format!("{}/stats?game_ids[]={}", get_nba_api_base_url(), game_id);
        
        let response = self.client.get(&url)
            .header(header::AUTHORIZATION, get_nba_api_key())
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get player stats: HTTP {}", response.status()));
        }
        
        let stats_response: PaginatedResponse<PlayerStats> = response.json().await?;
        Ok(stats_response.data)
    }
}

// Create a default instance for convenience
static NBA_API_INSTANCE: OnceLock<NbaApi> = OnceLock::new();

/// Get the NBA API instance
fn get_nba_api() -> &'static NbaApi {
    NBA_API_INSTANCE.get_or_init(|| NbaApi::new())
}

/// NBA Team information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: u32,
    pub abbreviation: String,
    pub city: String,
    pub conference: String,
    pub division: String,
    pub full_name: String,
    pub name: String,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({})\nConference: {}, Division: {}",
            self.city, self.name, self.abbreviation, self.conference, self.division
        )
    }
}

/// NBA Player information
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub height_feet: Option<u32>,
    pub height_inches: Option<u32>,
    pub weight_pounds: Option<u32>,
    pub team: Team,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = match (self.height_feet, self.height_inches) {
            (Some(feet), Some(inches)) => format!("{}'{}\"", feet, inches),
            _ => "Unknown".to_string(),
        };
        
        let weight = match self.weight_pounds {
            Some(pounds) => format!("{} lbs", pounds),
            None => "Unknown".to_string(),
        };
        
        write!(
            f,
            "{} {}\nPosition: {}\nHeight: {}\nWeight: {}\nTeam: {}",
            self.first_name, self.last_name, self.position, height, weight, self.team.full_name
        )
    }
}

/// NBA Game information
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: u32,
    pub date: String,
    pub home_team: Team,
    pub home_team_score: u32,
    pub period: u32,
    pub postseason: bool,
    pub season: u32,
    pub status: String,
    pub time: Option<String>,
    pub visitor_team: Team,
    pub visitor_team_score: u32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_status = if self.status == "Final" {
            "Final".to_string()
        } else {
            format!("In Progress - {} {}", self.period, self.time.as_deref().unwrap_or(""))
        };
        
        write!(
            f,
            "{} @ {} - {}\n{}: {}\n{}: {}\nSeason: {}, {}",
            self.visitor_team.abbreviation,
            self.home_team.abbreviation,
            self.date,
            self.visitor_team.name,
            self.visitor_team_score,
            self.home_team.name,
            self.home_team_score,
            self.season,
            game_status
        )
    }
}

/// Response structure for paginated results
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub next_cursor: Option<u32>,
    pub per_page: u32,
}

/// Single item response
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleResponse<T> {
    pub data: T,
}

/// NBA Player statistics for a game
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerStats {
    pub id: u32,
    pub min: Option<String>,
    pub fgm: Option<u32>,
    pub fga: Option<u32>,
    pub fg_pct: Option<f32>,
    pub fg3m: Option<u32>,
    pub fg3a: Option<u32>,
    pub fg3_pct: Option<f32>,
    pub ftm: Option<u32>,
    pub fta: Option<u32>,
    pub ft_pct: Option<f32>,
    pub oreb: Option<u32>,
    pub dreb: Option<u32>,
    pub reb: Option<u32>,
    pub ast: Option<u32>,
    pub stl: Option<u32>,
    pub blk: Option<u32>,
    pub turnover: Option<u32>,
    pub pf: Option<u32>,
    pub pts: Option<u32>,
    pub player: PlayerStatsPlayer,
    pub team: Team,
    pub game: GameSummary,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerStatsPlayer {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub team_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSummary {
    pub id: u32,
    pub date: String,
    pub home_team_id: u32,
    pub visitor_team_id: u32,
    pub home_team_score: u32,
    pub visitor_team_score: u32,
    pub season: u32,
    pub status: String,
    pub period: u32,
    pub time: Option<String>,
    pub postseason: bool,
}

impl fmt::Display for PlayerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = self.min.as_deref().unwrap_or("0");
        
        write!(
            f,
            "{} {}: {} pts, {} reb, {} ast, {} stl, {} blk, {}/{} FG, {}/{} 3PT, {}/{} FT in {} min",
            self.player.first_name,
            self.player.last_name,
            self.pts.unwrap_or(0),
            self.reb.unwrap_or(0),
            self.ast.unwrap_or(0),
            self.stl.unwrap_or(0),
            self.blk.unwrap_or(0),
            self.fgm.unwrap_or(0),
            self.fga.unwrap_or(0),
            self.fg3m.unwrap_or(0),
            self.fg3a.unwrap_or(0),
            self.ftm.unwrap_or(0),
            self.fta.unwrap_or(0),
            minutes
        )
    }
}

/// Format and display player statistics for a game, ordered by away team followed by home team
pub fn display_game_player_stats(_game_id: u32, stats: &[PlayerStats]) -> String {
    if stats.is_empty() {
        return "No player statistics available for this game.".to_string();
    }
    
    // Get the first stat to determine home and away team IDs
    let first_stat = &stats[0];
    let home_team_id = first_stat.game.home_team_id;
    let visitor_team_id = first_stat.game.visitor_team_id;
    
    // Group players by team
    let mut away_team_players: Vec<&PlayerStats> = Vec::new();
    let mut home_team_players: Vec<&PlayerStats> = Vec::new();
    
    for stat in stats {
        if stat.team.id == visitor_team_id {
            away_team_players.push(stat);
        } else if stat.team.id == home_team_id {
            home_team_players.push(stat);
        }
    }
    
    // Sort players by points scored (descending)
    away_team_players.sort_by(|a, b| b.pts.unwrap_or(0).cmp(&a.pts.unwrap_or(0)));
    home_team_players.sort_by(|a, b| b.pts.unwrap_or(0).cmp(&a.pts.unwrap_or(0)));
    
    let mut output = String::new();
    
    // Display away team stats
    if !away_team_players.is_empty() {
        output.push_str(&format!("\n{} ({}):\n", away_team_players[0].team.full_name, away_team_players[0].team.abbreviation));
        output.push_str("--------------------------------------------------\n");
        for player in &away_team_players {
            output.push_str(&format!("{}\n", player));
        }
    }
    
    // Display home team stats
    if !home_team_players.is_empty() {
        output.push_str(&format!("\n{} ({}):\n", home_team_players[0].team.full_name, home_team_players[0].team.abbreviation));
        output.push_str("--------------------------------------------------\n");
        for player in &home_team_players {
            output.push_str(&format!("{}\n", player));
        }
    }
    
    output
}

/// Create an authorized client for the balldontlie API
fn create_client() -> Result<Client> {
    let api_key = get_nba_api_key();
    
    if api_key.is_empty() {
        return Err(anyhow!("NBA API key is required. Please set the NBA_API_KEY environment variable."));
    }
    
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&api_key)?,
    );
    
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    
    Ok(client)
}

/// Get team information by ID
pub async fn get_team(team_id: u32) -> Result<Team> {
    get_nba_api().get_team(team_id).await
}

/// Get player information by ID
pub async fn get_player(player_id: u32) -> Result<Player> {
    get_nba_api().get_player(player_id).await
}

/// Get all NBA games for today
pub async fn get_todays_games() -> Result<Vec<Game>> {
    get_nba_api().get_todays_games().await
}

/// Get all NBA games from yesterday
pub async fn get_yesterdays_games() -> Result<Vec<Game>> {
    get_nba_api().get_yesterdays_games().await
}

/// Get games by date
#[allow(dead_code)]
pub async fn get_games_by_date(date: NaiveDate) -> Result<Vec<Game>> {
    get_nba_api().get_games_by_date(date).await
}

/// Get team games
pub async fn get_team_games(team_id: u32, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<Game>> {
    get_nba_api().get_team_games(team_id, start_date, end_date).await
}

/// Get player stats for a game
pub async fn get_game_player_stats(game_id: u32) -> Result<Vec<PlayerStats>> {
    get_nba_api().get_game_player_stats(game_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_team() -> Team {
        Team {
            id: 14,
            abbreviation: "LAL".to_string(),
            city: "Los Angeles".to_string(),
            conference: "West".to_string(),
            division: "Pacific".to_string(),
            full_name: "Los Angeles Lakers".to_string(),
            name: "Lakers".to_string(),
        }
    }
    
    fn mock_game() -> Game {
        Game {
            id: 12345,
            date: "2024-03-28".to_string(),
            home_team: mock_team(),
            home_team_score: 110,
            period: 4,
            postseason: false,
            season: 2023,
            status: "Final".to_string(),
            time: None,
            visitor_team: Team {
                id: 2,
                abbreviation: "BOS".to_string(),
                city: "Boston".to_string(),
                conference: "East".to_string(),
                division: "Atlantic".to_string(),
                full_name: "Boston Celtics".to_string(),
                name: "Celtics".to_string(),
            },
            visitor_team_score: 108,
        }
    }
    
    #[test]
    fn test_team_display() {
        let team = mock_team();
        let display = format!("{}", team);
        
        assert!(display.contains("Los Angeles Lakers"));
        assert!(display.contains("LAL"));
        assert!(display.contains("West"));
        assert!(display.contains("Pacific"));
    }
    
    #[test]
    fn test_game_display() {
        let game = mock_game();
        let display = format!("{}", game);
        
        assert!(display.contains("BOS @ LAL"));
        assert!(display.contains("Celtics: 108"));
        assert!(display.contains("Lakers: 110"));
        assert!(display.contains("Final"));
    }
} 