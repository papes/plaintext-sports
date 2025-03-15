use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use chrono::{Datelike, Local};
use std::sync::OnceLock;

static MLB_API_BASE_URL: OnceLock<String> = OnceLock::new();

fn get_mlb_api_base_url() -> Result<&'static str> {
    Ok(MLB_API_BASE_URL
        .get_or_init(|| "https://statsapi.mlb.com/api/v1".to_string())
        .as_str())
}

/// Represents the possible states of a game
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum GameState {
    Scheduled,
    Live,
    Final,
    Postponed,
    Cancelled,
    Suspended,
    #[serde(other)]
    Unknown,
}

impl GameState {
    /// Returns true if the game has been completed (i.e., is in the Final state)
    pub fn is_final(&self) -> bool {
        matches!(self, GameState::Final)
    }

    /// For compatibility with existing code that checks abstract_game_state
    pub fn abstract_game_state(&self) -> &str {
        match self {
            GameState::Final => "Final",
            GameState::Live => "Live",
            GameState::Scheduled => "Scheduled",
            GameState::Postponed => "Postponed",
            GameState::Cancelled => "Cancelled",
            GameState::Suspended => "Suspended",
            GameState::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Scheduled => write!(f, "Scheduled"),
            GameState::Live => write!(f, "Live"),
            GameState::Final => write!(f, "Final"),
            GameState::Postponed => write!(f, "Postponed"),
            GameState::Cancelled => write!(f, "Cancelled"),
            GameState::Suspended => write!(f, "Suspended"),
            GameState::Unknown => write!(f, "Unknown"),
        }
    }
}

/// MLB API client for making requests to the MLB Stats API
#[derive(Clone)]
pub struct MlbApi {
    client: Client,
}

/// Player information
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "primaryNumber")]
    pub primary_number: Option<String>,
    #[serde(rename = "currentTeam")]
    pub current_team: Option<Team>,
    #[serde(rename = "primaryPosition")]
    pub position: Option<Position>,
    #[serde(rename = "batSide")]
    pub bat_side: Option<BatSide>,
    #[serde(rename = "pitchHand")]
    pub pitch_hand: Option<PitchHand>,
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    #[serde(rename = "birthCity")]
    pub birth_city: Option<String>,
    #[serde(rename = "birthCountry")]
    pub birth_country: Option<String>,
    pub height: Option<String>,
    pub weight: Option<u32>,
    #[serde(rename = "currentAge")]
    pub current_age: Option<u32>,
    #[serde(rename = "mlbDebutDate")]
    pub mlb_debut_date: Option<String>,
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    pub active: Option<bool>,
}

/// Represents a Major League Baseball team with all its associated information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// Unique identifier for the team
    pub id: u32,
    /// Full name of the team (e.g., "New York Yankees")
    pub name: String,
    /// Short team code used in APIs and URLs
    #[serde(rename = "teamCode")]
    pub team_code: Option<String>,
    /// File code used for team assets
    #[serde(rename = "fileCode")]
    pub file_code: Option<String>,
    /// Team name without location (e.g., "Yankees")
    #[serde(rename = "teamName")]
    pub team_name: Option<String>,
    /// Location name (e.g., "New York")
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,
    /// Short version of team name
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    /// Team abbreviation (e.g., "NYY")
    pub abbreviation: Option<String>,
    /// Franchise name if different from team name
    #[serde(rename = "franchiseName")]
    pub franchise_name: Option<String>,
    /// Club name if different from team name
    #[serde(rename = "clubName")]
    pub club_name: Option<String>,
    /// First year the team played
    #[serde(rename = "firstYearOfPlay")]
    pub first_year_of_play: Option<String>,
    /// Whether the team is currently active
    pub active: Option<bool>,
    /// Team's home venue
    pub venue: Option<Venue>,
    /// League the team belongs to
    pub league: Option<League>,
    /// Division the team belongs to
    pub division: Option<Division>,
}

impl Team {
    /// Creates a new team with required fields
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            team_code: None,
            file_code: None,
            team_name: None,
            location_name: None,
            short_name: None,
            abbreviation: None,
            franchise_name: None,
            club_name: None,
            first_year_of_play: None,
            active: None,
            venue: None,
            league: None,
            division: None,
        }
    }

    /// Returns the team's display name, preferring full name over other variants
    pub fn display_name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns true if the team is currently active
    pub fn is_active(&self) -> bool {
        self.active.unwrap_or(true)
    }
}

/// Venue information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub id: u32,
    pub name: String,
}

/// League information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct League {
    pub id: u32,
    pub name: String,
}

/// Division information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Division {
    pub id: u32,
    pub name: String,
}

/// Player position
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub position_type: String,
    pub abbreviation: String,
}

/// Batting side
#[derive(Debug, Serialize, Deserialize)]
pub struct BatSide {
    pub code: String,
    pub description: String,
}

/// Pitching hand
#[derive(Debug, Serialize, Deserialize)]
pub struct PitchHand {
    pub code: String,
    pub description: String,
}

/// Represents a single MLB game with its associated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique identifier for the game
    #[serde(rename = "gamePk")]
    pub game_pk: u32,
    /// ISO 8601 formatted date and time of the game
    pub game_date: String,
    /// Current status of the game
    pub status: GameState,
    /// Teams participating in the game
    pub teams: GameTeams,
    /// Venue where the game is being played
    pub venue: Option<Venue>,
}

impl Game {
    /// Creates a new game with required fields
    pub fn new(
        game_pk: u32,
        game_date: String,
        status: GameState,
        teams: GameTeams,
        venue: Option<Venue>,
    ) -> Self {
        Self {
            game_pk,
            game_date,
            status,
            teams,
            venue,
        }
    }

    /// Returns true if the game has started
    pub fn has_started(&self) -> bool {
        matches!(self.status, GameState::Live | GameState::Final)
    }

    /// Returns true if the game is finished
    pub fn is_finished(&self) -> bool {
        matches!(self.status, GameState::Final)
    }

    /// Returns the winning team, if the game is finished
    pub fn winner(&self) -> Option<&Team> {
        if self.is_finished() {
            if self.teams.home.is_winner.unwrap_or(false) {
                Some(&self.teams.home.team)
            } else if self.teams.away.is_winner.unwrap_or(false) {
                Some(&self.teams.away.team)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Game teams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTeams {
    pub away: GameTeam,
    pub home: GameTeam,
}

/// Game team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTeam {
    pub score: Option<u32>,
    pub team: Team,
    #[serde(rename = "isWinner")]
    pub is_winner: Option<bool>,
}

/// Schedule information
#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub dates: Vec<ScheduleDate>,
}

/// Schedule date
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleDate {
    pub date: String,
    pub games: Vec<Game>,
}

/// Detailed game statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStats {
    pub away_team_stats: TeamStats,
    pub home_team_stats: TeamStats,
}

/// Team statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamStats {
    pub team_name: String,
    pub batting: BattingStats,
    pub pitching: PitchingStats,
    pub batters: Vec<PlayerBattingStats>,
    pub pitchers: Vec<PlayerPitchingStats>,
}

/// Batting statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct BattingStats {
    pub runs: u32,
    pub hits: u32,
    pub home_runs: u32,
    pub rbi: u32,
    pub stolen_bases: u32,
    pub avg: String,
    pub obp: String,
    pub slg: String,
    pub ops: String,
}

/// Pitching statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct PitchingStats {
    pub innings_pitched: String,
    pub hits_allowed: u32,
    pub runs_allowed: u32,
    pub earned_runs: u32,
    pub walks: u32,
    pub strikeouts: u32,
    pub home_runs_allowed: u32,
    pub era: String,
}

/// Player batting statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerBattingStats {
    pub name: String,
    pub hits: u32,
    pub at_bats: u32,
    pub home_runs: u32,
    pub rbi: u32,
    pub runs: u32,
    pub doubles: u32,
    pub triples: u32,
    pub stolen_bases: u32,
    pub walks: u32,
    pub strikeouts: u32,
    pub avg: Option<String>,
    pub obp: Option<String>,
    pub slg: Option<String>,
}

/// Player pitching statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerPitchingStats {
    pub name: String,
    pub innings_pitched: String,
    pub strikeouts: u32,
    pub earned_runs: u32,
    pub hits_allowed: u32,
    pub runs_allowed: u32,
    pub walks: u32,
    pub home_runs_allowed: u32,
    pub era: Option<String>,
}

/// Inning data with runs scored per inning
#[derive(Debug, Serialize, Deserialize)]
pub struct InningData {
    pub inning: u32,
    pub home: Option<u32>,
    pub away: Option<u32>,
}

/// Game with inning-by-inning breakdown
#[derive(Debug, Serialize, Deserialize)]
pub struct GameInnings {
    pub game_pk: u32,
    pub game_date: String,
    pub status: GameState,
    pub home_team: Team,
    pub away_team: Team,
    pub innings: Vec<InningData>,
    pub home_runs: Option<u32>,
    pub away_runs: Option<u32>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.full_name)?;
        if let Some(ref num) = self.primary_number {
            writeln!(f, "Number: {}", num)?;
        }
        if let Some(ref position) = self.position {
            writeln!(f, "Position: {} ({})", position.name, position.abbreviation)?;
        }
        if let Some(ref age) = self.current_age {
            writeln!(f, "Age: {}", age)?;
        }
        if let Some(ref birth_date) = self.birth_date {
            writeln!(f, "Birth Date: {}", birth_date)?;
        }
        if let Some(ref birth_city) = self.birth_city {
            if let Some(ref birth_country) = self.birth_country {
                writeln!(f, "Birthplace: {}, {}", birth_city, birth_country)?;
            }
        }
        if let Some(ref height) = self.height {
            if let Some(weight) = self.weight {
                writeln!(f, "Height/Weight: {} / {} lbs", height, weight)?;
            }
        }
        if let Some(ref bat_side) = self.bat_side {
            writeln!(f, "Bats: {}", bat_side.description)?;
        }
        if let Some(ref pitch_hand) = self.pitch_hand {
            writeln!(f, "Throws: {}", pitch_hand.description)?;
        }
        if let Some(ref debut) = self.mlb_debut_date {
            writeln!(f, "MLB Debut: {}", debut)?;
        }
        if let Some(ref nickname) = self.nick_name {
            writeln!(f, "Nickname: {}", nickname)?;
        }
        if let Some(active) = self.active {
            writeln!(f, "Active: {}", if active { "Yes" } else { "No" })?;
        }
        Ok(())
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Team: {}", self.name)?;
        
        if let Some(ref location) = self.location_name {
            if let Some(ref team_name) = self.team_name {
                writeln!(f, "Location/Name: {} {}", location, team_name)?;
            }
        }
        
        if let Some(ref abbr) = self.abbreviation {
            writeln!(f, "Abbreviation: {}", abbr)?;
        }
        
        if let Some(ref venue) = self.venue {
            writeln!(f, "Venue: {}", venue.name)?;
        }
        
        if let Some(ref league) = self.league {
            if let Some(ref division) = self.division {
                writeln!(f, "League/Division: {} / {}", league.name, division.name)?;
            }
        }
        
        if let Some(ref first_year) = self.first_year_of_play {
            writeln!(f, "First Year of Play: {}", first_year)?;
        }
        
        if let Some(active) = self.active {
            writeln!(f, "Active: {}", if active { "Yes" } else { "No" })?;
        }
        
        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Game ID: {}", self.game_pk)?;
        writeln!(f, "Date: {}", self.game_date.split('T').next().unwrap_or(&self.game_date))?;
        writeln!(f, "Status: {}", self.status)?;
        writeln!(f, "Teams:")?;
        
        let away_location = self.teams.away.team.location_name.as_deref().unwrap_or("");
        let home_location = self.teams.home.team.location_name.as_deref().unwrap_or("");
        
        writeln!(f, "  Away: {} {} ({})", away_location, self.teams.away.team.name, self.teams.away.score.unwrap_or(0))?;
        writeln!(f, "  Home: {} {} ({})", home_location, self.teams.home.team.name, self.teams.home.score.unwrap_or(0))?;
        
        if let Some(ref venue) = self.venue {
            writeln!(f, "Venue: {}", venue.name)?;
        }
        Ok(())
    }
}

impl MlbApi {
    /// Create a new MLB API client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Get player information by ID
    pub async fn get_player(&self, player_id: u32) -> Result<Player> {
        let url = format!("{}/people/{}", get_mlb_api_base_url()?, player_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch player data: HTTP {}", response.status()));
        }
        
        let data: Value = response.json().await?;
        
        let people = data.get("people").ok_or_else(|| -> anyhow::Error {
            anyhow!("Player with ID {} not found", player_id)
        })?;
        
        let player = people.get(0).ok_or_else(|| -> anyhow::Error {
            anyhow!("Player with ID {} not found", player_id)
        })?;

        let player: Player = serde_json::from_value(player.to_owned())?;
        Ok(player)
    }

    /// Get team information by ID
    pub async fn get_team(&self, team_id: u32) -> Result<Team> {
        let url = format!("{}/teams/{}", get_mlb_api_base_url()?, team_id);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch team data: HTTP {}", response.status()));
        }

        let data = response.json::<serde_json::Value>().await?;
        let teams = data.get("teams").ok_or_else(|| -> anyhow::Error {
            anyhow!("Team with ID {} not found", team_id)
        })?;
        let team = teams.get(0).ok_or_else(|| -> anyhow::Error {
            anyhow!("Team with ID {} not found", team_id)
        })?;

        let team: Team = serde_json::from_value(team.to_owned())?;
        Ok(team)
    }

    /// Get schedule for a team
    pub async fn get_team_schedule(&self, team_id: u32, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<Game>> {
        // Default to current month if no dates provided
        let now = Local::now();
        let start = start_date.unwrap_or_else(|| format!("{}-{:02}-01", now.year(), now.month()));
        let end = end_date.unwrap_or_else(|| format!("{}-{:02}-30", now.year(), now.month()));
        
        let url = format!(
            "{}/schedule?teamId={}&startDate={}&endDate={}&sportId=1",
            get_mlb_api_base_url()?, team_id, start, end
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch schedule data: HTTP {}", response.status()));
        }
        
        let schedule: Schedule = response.json().await?;
        
        let mut games = Vec::new();
        for date in schedule.dates {
            for game in date.games {
                games.push(game);
            }
        }
        
        Ok(games)
    }

    /// Get game information by ID
    pub async fn get_game(&self, game_id: u64) -> Result<Game> {
        let url = format!("{}/game/{}/feed/live", get_mlb_api_base_url()?, game_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch game data: HTTP {}", response.status()));
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let game_data = data.get("gameData").ok_or_else(|| anyhow!("Missing game data"))?;
        
        // Extract basic game information
        let status_value = &game_data["status"];
        let status_abstract_game_state = status_value.get("abstractGameState").and_then(|s| s.as_str()).unwrap_or("Unknown");
        let status_detailed_state = status_value.get("detailedState").and_then(|s| s.as_str()).unwrap_or("Unknown");
        println!("DEBUG: Game {} status: abstractGameState='{}', detailedState='{}'", 
                 game_data["gamePk"].as_u64().unwrap_or(0),
                 status_abstract_game_state,
                 status_detailed_state);
        
        // Map the abstractGameState to our GameState enum
        let status = match status_abstract_game_state {
            "Final" => GameState::Final,
            "Live" => GameState::Live,
            "Preview" => GameState::Scheduled,
            "Postponed" => GameState::Postponed,
            "Cancelled" => GameState::Cancelled,
            "Suspended" => GameState::Suspended,
            _ => GameState::Unknown,
        };
        
        let game = Game {
            game_pk: game_data["gamePk"].as_u64().unwrap_or(0) as u32,
            game_date: game_data["gameDate"].as_str().unwrap_or("").to_string(),
            status, // Use our mapped status
            teams: GameTeams {
                away: self.extract_game_team(&data, "away")?,
                home: self.extract_game_team(&data, "home")?,
            },
            venue: Some(Venue {
                id: game_data["venue"]["id"].as_u64().unwrap_or(0) as u32,
                name: game_data["venue"]["name"].as_str().unwrap_or("").to_string(),
            }),
        };
        Ok(game)
    }

    /// Helper method to extract team information from game data
    fn extract_game_team(&self, data: &Value, team_type: &str) -> Result<GameTeam> {
        let team_data = data.get(team_type).ok_or_else(|| 
            anyhow!("Missing {} team data", team_type)
        )?;
        
        Ok(GameTeam {
            score: team_data["runs"].as_u64().map(|s| s as u32),
            team: Team {
                id: team_data["id"].as_u64().unwrap_or(0) as u32,
                name: team_data["name"].as_str().unwrap_or("").to_string(),
                team_code: team_data["teamCode"].as_str().map(String::from),
                file_code: team_data["fileCode"].as_str().map(String::from),
                team_name: team_data["teamName"].as_str().map(String::from),
                location_name: team_data["locationName"].as_str().map(String::from),
                short_name: team_data["shortName"].as_str().map(String::from),
                abbreviation: team_data["abbreviation"].as_str().map(String::from),
                franchise_name: None,
                club_name: None,
                first_year_of_play: None,
                active: None,
                venue: None,
                league: None,
                division: None,
            },
            is_winner: team_data["isWinner"].as_bool(),
        })
    }

    /// Get all games scheduled for today
    pub async fn get_todays_games(&self) -> Result<Vec<Game>> {
        let today = Local::now().date_naive();
        let url = format!(
            "{}/schedule?sportId=1&date={}-{}-{}&hydrate=game(content(editorial(recap))),linescore,team",
            get_mlb_api_base_url()?,
            today.year(),
            today.month(),
            today.day(),
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch today's games: HTTP {}", response.status()));
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let empty_vec = Vec::new();
        let dates = data.get("dates").and_then(|d| d.as_array()).unwrap_or(&empty_vec);
        
        let mut games = Vec::new();
        for date in dates {
            if let Some(games_array) = date.get("games").and_then(|g| g.as_array()) {
                for game_data in games_array {
                    // Add debug information about the game status
                    let status_value = &game_data["status"];
                    let status_abstract_game_state = status_value.get("abstractGameState").and_then(|s| s.as_str()).unwrap_or("Unknown");
                    let status_detailed_state = status_value.get("detailedState").and_then(|s| s.as_str()).unwrap_or("Unknown");
                    println!("DEBUG: Game {} status: abstractGameState='{}', detailedState='{}'", 
                             game_data["gamePk"].as_u64().unwrap_or(0),
                             status_abstract_game_state,
                             status_detailed_state);
                    
                    // Map the abstractGameState to our GameState enum
                    let status = match status_abstract_game_state {
                        "Final" => GameState::Final,
                        "Live" => GameState::Live,
                        "Preview" => GameState::Scheduled,
                        "Postponed" => GameState::Postponed,
                        "Cancelled" => GameState::Cancelled,
                        "Suspended" => GameState::Suspended,
                        _ => GameState::Unknown,
                    };
                    
                    let game = Game {
                        game_pk: game_data["gamePk"].as_u64().unwrap_or(0) as u32,
                        game_date: game_data["gameDate"].as_str().unwrap_or("").to_string(),
                        status,
                        teams: GameTeams {
                            away: GameTeam {
                                score: game_data["teams"]["away"]["score"].as_u64().map(|s| s as u32),
                                team: Team {
                                    id: game_data["teams"]["away"]["team"]["id"].as_u64().unwrap_or(0) as u32,
                                    name: game_data["teams"]["away"]["team"]["name"].as_str().unwrap_or("").to_string(),
                                    team_code: game_data["teams"]["away"]["team"]["teamCode"].as_str().map(String::from),
                                    file_code: game_data["teams"]["away"]["team"]["fileCode"].as_str().map(String::from),
                                    team_name: game_data["teams"]["away"]["team"]["teamName"].as_str().map(String::from),
                                    location_name: game_data["teams"]["away"]["team"]["locationName"].as_str().map(String::from),
                                    short_name: game_data["teams"]["away"]["team"]["shortName"].as_str().map(String::from),
                                    abbreviation: game_data["teams"]["away"]["team"]["abbreviation"].as_str().map(String::from),
                                    franchise_name: None,
                                    club_name: None,
                                    first_year_of_play: None,
                                    active: None,
                                    venue: None,
                                    league: None,
                                    division: None,
                                },
                                is_winner: game_data["teams"]["away"]["isWinner"].as_bool(),
                            },
                            home: GameTeam {
                                score: game_data["teams"]["home"]["score"].as_u64().map(|s| s as u32),
                                team: Team {
                                    id: game_data["teams"]["home"]["team"]["id"].as_u64().unwrap_or(0) as u32,
                                    name: game_data["teams"]["home"]["team"]["name"].as_str().unwrap_or("").to_string(),
                                    team_code: game_data["teams"]["home"]["team"]["teamCode"].as_str().map(String::from),
                                    file_code: game_data["teams"]["home"]["team"]["fileCode"].as_str().map(String::from),
                                    team_name: game_data["teams"]["home"]["team"]["teamName"].as_str().map(String::from),
                                    location_name: game_data["teams"]["home"]["team"]["locationName"].as_str().map(String::from),
                                    short_name: game_data["teams"]["home"]["team"]["shortName"].as_str().map(String::from),
                                    abbreviation: game_data["teams"]["home"]["team"]["abbreviation"].as_str().map(String::from),
                                    franchise_name: None,
                                    club_name: None,
                                    first_year_of_play: None,
                                    active: None,
                                    venue: None,
                                    league: None,
                                    division: None,
                                },
                                is_winner: game_data["teams"]["home"]["isWinner"].as_bool(),
                            },
                        },
                        venue: Some(Venue {
                            id: game_data["venue"]["id"].as_u64().unwrap_or(0) as u32,
                            name: game_data["venue"]["name"].as_str().unwrap_or("").to_string(),
                        }),
                    };
                    games.push(game);
                }
            }
        }
        
        Ok(games)
    }

    /// Get all games scheduled for yesterday
    pub async fn get_yesterdays_games(&self) -> Result<Vec<Game>> {
        // Get yesterday's date in YYYY-MM-DD format
        let yesterday = Local::now().checked_sub_days(chrono::Days::new(1)).unwrap().format("%Y-%m-%d").to_string();
        
        let url = format!(
            "{}/schedule?sportId=1&date={}&hydrate=game(content(editorial(recap))),linescore,team",
            get_mlb_api_base_url()?, yesterday
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch yesterday's games: HTTP {}", response.status()));
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let empty_vec = Vec::new();
        let dates = data.get("dates").and_then(|d| d.as_array()).unwrap_or(&empty_vec);
        
        let mut games = Vec::new();
        for date in dates {
            if let Some(games_array) = date.get("games").and_then(|g| g.as_array()) {
                for game_data in games_array {
                    // Add debug information about the game status
                    let status_value = &game_data["status"];
                    let status_abstract_game_state = status_value.get("abstractGameState").and_then(|s| s.as_str()).unwrap_or("Unknown");
                    let status_detailed_state = status_value.get("detailedState").and_then(|s| s.as_str()).unwrap_or("Unknown");
                    println!("DEBUG: Game {} status: abstractGameState='{}', detailedState='{}'", 
                             game_data["gamePk"].as_u64().unwrap_or(0),
                             status_abstract_game_state,
                             status_detailed_state);
                    
                    // Map the abstractGameState to our GameState enum
                    let status = match status_abstract_game_state {
                        "Final" => GameState::Final,
                        "Live" => GameState::Live,
                        "Preview" => GameState::Scheduled,
                        "Postponed" => GameState::Postponed,
                        "Cancelled" => GameState::Cancelled,
                        "Suspended" => GameState::Suspended,
                        _ => GameState::Unknown,
                    };
                    
                    let game = Game {
                        game_pk: game_data["gamePk"].as_u64().unwrap_or(0) as u32,
                        game_date: game_data["gameDate"].as_str().unwrap_or("").to_string(),
                        status,
                        teams: GameTeams {
                            away: GameTeam {
                                score: game_data["teams"]["away"]["score"].as_u64().map(|s| s as u32),
                                team: Team {
                                    id: game_data["teams"]["away"]["team"]["id"].as_u64().unwrap_or(0) as u32,
                                    name: game_data["teams"]["away"]["team"]["name"].as_str().unwrap_or("").to_string(),
                                    team_code: game_data["teams"]["away"]["team"]["teamCode"].as_str().map(String::from),
                                    file_code: game_data["teams"]["away"]["team"]["fileCode"].as_str().map(String::from),
                                    team_name: game_data["teams"]["away"]["team"]["teamName"].as_str().map(String::from),
                                    location_name: game_data["teams"]["away"]["team"]["locationName"].as_str().map(String::from),
                                    short_name: game_data["teams"]["away"]["team"]["shortName"].as_str().map(String::from),
                                    abbreviation: game_data["teams"]["away"]["team"]["abbreviation"].as_str().map(String::from),
                                    franchise_name: None,
                                    club_name: None,
                                    first_year_of_play: None,
                                    active: None,
                                    venue: None,
                                    league: None,
                                    division: None,
                                },
                                is_winner: game_data["teams"]["away"]["isWinner"].as_bool(),
                            },
                            home: GameTeam {
                                score: game_data["teams"]["home"]["score"].as_u64().map(|s| s as u32),
                                team: Team {
                                    id: game_data["teams"]["home"]["team"]["id"].as_u64().unwrap_or(0) as u32,
                                    name: game_data["teams"]["home"]["team"]["name"].as_str().unwrap_or("").to_string(),
                                    team_code: game_data["teams"]["home"]["team"]["teamCode"].as_str().map(String::from),
                                    file_code: game_data["teams"]["home"]["team"]["fileCode"].as_str().map(String::from),
                                    team_name: game_data["teams"]["home"]["team"]["teamName"].as_str().map(String::from),
                                    location_name: game_data["teams"]["home"]["team"]["locationName"].as_str().map(String::from),
                                    short_name: game_data["teams"]["home"]["team"]["shortName"].as_str().map(String::from),
                                    abbreviation: game_data["teams"]["home"]["team"]["abbreviation"].as_str().map(String::from),
                                    franchise_name: None,
                                    club_name: None,
                                    first_year_of_play: None,
                                    active: None,
                                    venue: None,
                                    league: None,
                                    division: None,
                                },
                                is_winner: game_data["teams"]["home"]["isWinner"].as_bool(),
                            },
                        },
                        venue: Some(Venue {
                            id: game_data["venue"]["id"].as_u64().unwrap_or(0) as u32,
                            name: game_data["venue"]["name"].as_str().unwrap_or("").to_string(),
                        }),
                    };
                    games.push(game);
                }
            }
        }
        
        Ok(games)
    }

    /// Get inning-by-inning data for a game
    pub async fn get_game_innings(&self, game_id: u32) -> Result<GameInnings> {
        // Try each endpoint in sequence
        match self.try_feed_live_endpoint(game_id).await {
            Ok(innings) => return Ok(innings),
            Err(feed_live_error) => {
                // Feed/live endpoint failed, try linescore endpoint
                match self.try_linescore_endpoint(game_id).await {
                    Ok(innings) => return Ok(innings),
                    Err(linescore_error) => {
                        // Linescore endpoint failed, try playByPlay endpoint
                        match self.try_playbyplay_endpoint(game_id).await {
                            Ok(innings) => return Ok(innings),
                            Err(playbyplay_error) => {
                                // All endpoints failed, return a comprehensive error
                                return Err(anyhow!(
                                    "Could not retrieve inning data from any endpoint. Errors: feed/live: {}, linescore: {}, playByPlay: {}",
                                    feed_live_error, linescore_error, playbyplay_error
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Try to get inning data from the feed/live endpoint
    async fn try_feed_live_endpoint(&self, game_id: u32) -> Result<GameInnings> {
        let feed_live_url = format!("{}/game/{}/feed/live", get_mlb_api_base_url()?, game_id);
        let response = self.client.get(&feed_live_url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }
        
        let data = response.json::<serde_json::Value>().await?;
        
        // Check if this is a spring training game
        let game_type = data.get("gameData")
            .and_then(|gd| gd.get("game"))
            .and_then(|g| g.get("type"))
            .and_then(|t| t.as_str())
            .unwrap_or("unknown");
        
        if game_type == "S" {
            // Spring training game
            println!("This is a spring training game (type: {}). Inning-by-inning data may be limited.", game_type);
        }
        
        // Continue with the existing logic for processing feed/live data
        let game_data = data.get("gameData").ok_or_else(|| anyhow!("Missing game data"))?;
        
        // Extract game status information
        let status_value = game_data.get("status").ok_or_else(|| anyhow!("Missing status data"))?;
        let status_abstract_game_state = status_value.get("abstractGameState").and_then(|s| s.as_str()).unwrap_or("Unknown");
        let status_detailed_state = status_value.get("detailedState").and_then(|s| s.as_str()).unwrap_or("Unknown");
        println!("DEBUG: Game {} innings - status: abstractGameState='{}', detailedState='{}'", 
                game_id,
                status_abstract_game_state,
                status_detailed_state);
        
        // Map the abstractGameState to our GameState enum
        let status = match status_abstract_game_state {
            "Final" => GameState::Final,
            "Live" => GameState::Live,
            "Preview" => GameState::Scheduled,
            "Postponed" => GameState::Postponed,
            "Cancelled" => GameState::Cancelled,
            "Suspended" => GameState::Suspended,
            _ => GameState::Unknown,
        };
        
        let linescore = data.get("liveData").ok_or_else(|| anyhow!("Missing live data"))?
            .get("linescore").ok_or_else(|| anyhow!("Missing linescore data"))?;
        
        // Extract teams data
        let teams = linescore.get("teams").ok_or_else(|| anyhow!("Missing teams data"))?;
        let away = teams.get("away").ok_or_else(|| anyhow!("Missing away team data"))?;
        let home = teams.get("home").ok_or_else(|| anyhow!("Missing home team data"))?;
        
        // Extract home and away teams
        let home_team = Team {
            id: game_data["teams"]["home"]["id"].as_u64().unwrap_or(0) as u32,
            name: game_data["teams"]["home"]["name"].as_str().unwrap_or("").to_string(),
            team_code: game_data["teams"]["home"]["teamCode"].as_str().map(String::from),
            file_code: game_data["teams"]["home"]["fileCode"].as_str().map(String::from),
            team_name: game_data["teams"]["home"]["teamName"].as_str().map(String::from),
            location_name: game_data["teams"]["home"]["locationName"].as_str().map(String::from),
            short_name: game_data["teams"]["home"]["shortName"].as_str().map(String::from),
            abbreviation: game_data["teams"]["home"]["abbreviation"].as_str().map(String::from),
            franchise_name: None,
            club_name: None,
            first_year_of_play: None,
            active: None,
            venue: None,
            league: None,
            division: None,
        };
        
        let away_team = Team {
            id: game_data["teams"]["away"]["id"].as_u64().unwrap_or(0) as u32,
            name: game_data["teams"]["away"]["name"].as_str().unwrap_or("").to_string(),
            team_code: game_data["teams"]["away"]["teamCode"].as_str().map(String::from),
            file_code: game_data["teams"]["away"]["fileCode"].as_str().map(String::from),
            team_name: game_data["teams"]["away"]["teamName"].as_str().map(String::from),
            location_name: game_data["teams"]["away"]["locationName"].as_str().map(String::from),
            short_name: game_data["teams"]["away"]["shortName"].as_str().map(String::from),
            abbreviation: game_data["teams"]["away"]["abbreviation"].as_str().map(String::from),
            franchise_name: None,
            club_name: None,
            first_year_of_play: None,
            active: None,
            venue: None,
            league: None,
            division: None,
        };
        
        // Extract innings data
        let mut innings = Vec::new();
        let innings_data = linescore.get("innings").and_then(|v| v.as_array());
        
        if let Some(innings_array) = innings_data {
            for (i, inning) in innings_array.iter().enumerate() {
                let inning_num = i as u32 + 1;
                let home_runs = inning["home"]["runs"].as_u64().map(|r| r as u32);
                let away_runs = inning["away"]["runs"].as_u64().map(|r| r as u32);
                
                innings.push(InningData {
                    inning: inning_num,
                    home: home_runs,
                    away: away_runs,
                });
            }
        }
        
        // Extract total runs
        let home_runs = home["runs"].as_u64().map(|r| r as u32);
        let away_runs = away["runs"].as_u64().map(|r| r as u32);
        
        // Extract game info
        let game_pk = game_data["game"]["pk"].as_u64().unwrap_or(0) as u32;
        let game_date = game_data["datetime"]["dateTime"].as_str().unwrap_or("").to_string();
        
        Ok(GameInnings {
            game_pk,
            game_date,
            status,
            home_team,
            away_team,
            innings,
            home_runs,
            away_runs,
        })
    }

    /// Try to get inning data from the linescore endpoint
    async fn try_linescore_endpoint(&self, game_id: u32) -> Result<GameInnings> {
        println!("DEBUG: Using /linescore endpoint for game {}", game_id);
        
        let linescore_url = format!("{}/game/{}/linescore", get_mlb_api_base_url()?, game_id);
        let linescore_response = self.client.get(&linescore_url).send().await?;
        
        if !linescore_response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", linescore_response.status()));
        }
        
        // Get basic game info from the boxscore endpoint since we know that's working
        let boxscore_url = format!("{}/game/{}/boxscore", get_mlb_api_base_url()?, game_id);
        let boxscore_response = self.client.get(&boxscore_url).send().await?;
        
        if !boxscore_response.status().is_success() {
            return Err(anyhow!("Failed to fetch game data from boxscore: HTTP {}", boxscore_response.status()));
        }
        
        let boxscore_data = boxscore_response.json::<serde_json::Value>().await?;
        
        // Process linescore data
        let linescore_data = linescore_response.json::<serde_json::Value>().await?;
        
        // Extract teams
        let teams = linescore_data.get("teams").ok_or_else(|| anyhow!("Missing teams data in linescore"))?;
        let away = teams.get("away").ok_or_else(|| anyhow!("Missing away team data in linescore"))?;
        let home = teams.get("home").ok_or_else(|| anyhow!("Missing home team data in linescore"))?;
        
        // Extract home and away teams from boxscore
        let home_team = Team {
            id: boxscore_data["teams"]["home"]["team"]["id"].as_u64().unwrap_or(0) as u32,
            name: boxscore_data["teams"]["home"]["team"]["name"].as_str().unwrap_or("").to_string(),
            team_code: None,
            file_code: None,
            team_name: None,
            location_name: None,
            short_name: None,
            abbreviation: None,
            franchise_name: None,
            club_name: None,
            first_year_of_play: None,
            active: None,
            venue: None,
            league: None,
            division: None,
        };
        
        let away_team = Team {
            id: boxscore_data["teams"]["away"]["team"]["id"].as_u64().unwrap_or(0) as u32,
            name: boxscore_data["teams"]["away"]["team"]["name"].as_str().unwrap_or("").to_string(),
            team_code: None,
            file_code: None,
            team_name: None,
            location_name: None,
            short_name: None,
            abbreviation: None,
            franchise_name: None,
            club_name: None,
            first_year_of_play: None,
            active: None,
            venue: None,
            league: None,
            division: None,
        };
        
        // Assume it's a completed game if we're getting linescore data for a game we know is done
        let status = GameState::Final;
        
        // Extract innings data
        let mut innings = Vec::new();
        let innings_data = linescore_data.get("innings").and_then(|v| v.as_array());
        
        if let Some(innings_array) = innings_data {
            for (i, inning) in innings_array.iter().enumerate() {
                let inning_num = i as u32 + 1;
                let home_runs = inning["home"]["runs"].as_u64().map(|r| r as u32);
                let away_runs = inning["away"]["runs"].as_u64().map(|r| r as u32);
                
                innings.push(InningData {
                    inning: inning_num,
                    home: home_runs,
                    away: away_runs,
                });
            }
        }
        
        // Extract total runs
        let home_runs = home["runs"].as_u64().map(|r| r as u32);
        let away_runs = away["runs"].as_u64().map(|r| r as u32);
        
        // Get game date from info object if available
        let game_date = linescore_data.get("gameDate")
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();
        
        Ok(GameInnings {
            game_pk: game_id,
            game_date,
            status,
            home_team,
            away_team,
            innings,
            home_runs,
            away_runs,
        })
    }

    /// Try to get inning data from the playByPlay endpoint
    async fn try_playbyplay_endpoint(&self, game_id: u32) -> Result<GameInnings> {
        println!("DEBUG: Using /playByPlay endpoint for game {}", game_id);
        
        let playbyplay_url = format!("{}/game/{}/playByPlay", get_mlb_api_base_url()?, game_id);
        let playbyplay_response = self.client.get(&playbyplay_url).send().await?;
        
        if !playbyplay_response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", playbyplay_response.status()));
        }
        
        // For now, we'll return an error since we haven't implemented this endpoint yet
        Err(anyhow!("Play-by-play endpoint found but not implemented for inning extraction"))
    }

    /// Get game statistics
    pub async fn get_game_stats(&self, game_id: u32) -> Result<GameStats> {
        let url = format!("{}/game/{}/boxscore", get_mlb_api_base_url()?, game_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch game stats: HTTP {}", response.status()));
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let teams = data.get("teams").ok_or_else(|| anyhow!("Missing teams data"))?;
        let away = teams.get("away").ok_or_else(|| anyhow!("Missing away team data"))?;
        let home = teams.get("home").ok_or_else(|| anyhow!("Missing home team data"))?;
        
        // Extract team stats
        let away_team_stats = self.extract_team_stats(away)?;
        let home_team_stats = self.extract_team_stats(home)?;
        
        Ok(GameStats {
            away_team_stats,
            home_team_stats,
        })
    }

    /// Helper method to extract team statistics
    fn extract_team_stats(&self, team_data: &Value) -> Result<TeamStats> {
        // Get team info
        let team_name = team_data.get("team").and_then(|t| t.get("name"))
            .and_then(|n| n.as_str())
            .ok_or_else(|| anyhow!("Missing team name"))?
            .to_string();
            
        // Extract batting stats
        let batting = BattingStats {
            runs: team_data["teamStats"]["batting"]["runs"].as_u64().unwrap_or(0) as u32,
            hits: team_data["teamStats"]["batting"]["hits"].as_u64().unwrap_or(0) as u32,
            home_runs: team_data["teamStats"]["batting"]["homeRuns"].as_u64().unwrap_or(0) as u32,
            rbi: team_data["teamStats"]["batting"]["rbi"].as_u64().unwrap_or(0) as u32,
            stolen_bases: team_data["teamStats"]["batting"]["stolenBases"].as_u64().unwrap_or(0) as u32,
            avg: team_data["teamStats"]["batting"]["avg"].as_str().unwrap_or(".000").to_string(),
            obp: team_data["teamStats"]["batting"]["obp"].as_str().unwrap_or(".000").to_string(),
            slg: team_data["teamStats"]["batting"]["slg"].as_str().unwrap_or(".000").to_string(),
            ops: team_data["teamStats"]["batting"]["ops"].as_str().unwrap_or(".000").to_string(),
        };
        
        // Extract pitching stats
        let pitching = PitchingStats {
            innings_pitched: team_data["teamStats"]["pitching"]["inningsPitched"].as_str().unwrap_or("0").to_string(),
            hits_allowed: team_data["teamStats"]["pitching"]["hits"].as_u64().unwrap_or(0) as u32,
            runs_allowed: team_data["teamStats"]["pitching"]["runs"].as_u64().unwrap_or(0) as u32,
            earned_runs: team_data["teamStats"]["pitching"]["earnedRuns"].as_u64().unwrap_or(0) as u32,
            walks: team_data["teamStats"]["pitching"]["baseOnBalls"].as_u64().unwrap_or(0) as u32,
            strikeouts: team_data["teamStats"]["pitching"]["strikeOuts"].as_u64().unwrap_or(0) as u32,
            home_runs_allowed: team_data["teamStats"]["pitching"]["homeRuns"].as_u64().unwrap_or(0) as u32,
            era: team_data["teamStats"]["pitching"]["era"].as_str().unwrap_or("0.00").to_string(),
        };
        
        // Debug print to check if batters and pitchers data exists in the response
        println!("DEBUG: Batters data exists: {}", team_data.get("batters").is_some());
        if let Some(batters_array) = team_data.get("batters") {
            println!("DEBUG: Batters array is array: {}", batters_array.is_array());
            if let Some(arr) = batters_array.as_array() {
                println!("DEBUG: Batters array length: {}", arr.len());
                if !arr.is_empty() {
                    // Print the first batter object to see the structure
                    println!("DEBUG: First batter structure: {}", serde_json::to_string_pretty(&arr[0]).unwrap_or_else(|_| "Error serializing".to_string()));
                }
            }
        }
        
        // Extract batter stats
        let batters = if let Some(batters_array) = team_data["batters"].as_array() {
            let mut batter_stats = Vec::new();
            for batter in batters_array {
                // Try to extract player ID to find corresponding player details
                if let Some(batter_id) = batter.as_u64() {
                    // Look for player details in the players map
                    if let Some(players) = team_data.get("players") {
                        // Construct the player key
                        let player_key = format!("ID{}", batter_id);
                        if let Some(player_obj) = players.get(&player_key) {
                            // Extract player name
                            let name = player_obj.get("person").and_then(|p| p.get("fullName")).and_then(|n| n.as_str()).unwrap_or("Unknown Player").to_string();
                            
                            // Check if stats are available for this player
                            if let Some(stats_obj) = player_obj.get("stats").and_then(|s| s.get("batting")) {
                                batter_stats.push(PlayerBattingStats {
                                    name,
                                    hits: stats_obj.get("hits").and_then(|h| h.as_u64()).unwrap_or(0) as u32,
                                    at_bats: stats_obj.get("atBats").and_then(|ab| ab.as_u64()).unwrap_or(0) as u32,
                                    home_runs: stats_obj.get("homeRuns").and_then(|hr| hr.as_u64()).unwrap_or(0) as u32,
                                    rbi: stats_obj.get("rbi").and_then(|rbi| rbi.as_u64()).unwrap_or(0) as u32,
                                    runs: stats_obj.get("runs").and_then(|r| r.as_u64()).unwrap_or(0) as u32,
                                    doubles: stats_obj.get("doubles").and_then(|d| d.as_u64()).unwrap_or(0) as u32,
                                    triples: stats_obj.get("triples").and_then(|t| t.as_u64()).unwrap_or(0) as u32,
                                    stolen_bases: stats_obj.get("stolenBases").and_then(|sb| sb.as_u64()).unwrap_or(0) as u32,
                                    walks: stats_obj.get("baseOnBalls").and_then(|bb| bb.as_u64()).unwrap_or(0) as u32,
                                    strikeouts: stats_obj.get("strikeOuts").and_then(|so| so.as_u64()).unwrap_or(0) as u32,
                                    avg: stats_obj.get("avg").and_then(|avg| avg.as_str()).map(String::from),
                                    obp: stats_obj.get("obp").and_then(|obp| obp.as_str()).map(String::from),
                                    slg: stats_obj.get("slg").and_then(|slg| slg.as_str()).map(String::from),
                                });
                            }
                        }
                    }
                }
            }
            batter_stats
        } else {
            println!("DEBUG: No batters array found in team data");
            Vec::new()
        };
        
        // Debug print pitcher data
        println!("DEBUG: Pitchers data exists: {}", team_data.get("pitchers").is_some());
        if let Some(pitchers_array) = team_data.get("pitchers") {
            if let Some(arr) = pitchers_array.as_array() {
                if !arr.is_empty() {
                    // Print the first pitcher object to see the structure
                    println!("DEBUG: First pitcher structure: {}", serde_json::to_string_pretty(&arr[0]).unwrap_or_else(|_| "Error serializing".to_string()));
                }
            }
        }
        
        // Extract pitcher stats
        let pitchers = if let Some(pitchers_array) = team_data["pitchers"].as_array() {
            let mut pitcher_stats = Vec::new();
            for pitcher in pitchers_array {
                // Try to extract player ID to find corresponding player details
                if let Some(pitcher_id) = pitcher.as_u64() {
                    // Look for player details in the players map
                    if let Some(players) = team_data.get("players") {
                        // Construct the player key
                        let player_key = format!("ID{}", pitcher_id);
                        if let Some(player_obj) = players.get(&player_key) {
                            // Extract player name
                            let name = player_obj.get("person").and_then(|p| p.get("fullName")).and_then(|n| n.as_str()).unwrap_or("Unknown Player").to_string();
                            
                            // Check if stats are available for this player
                            if let Some(stats_obj) = player_obj.get("stats").and_then(|s| s.get("pitching")) {
                                pitcher_stats.push(PlayerPitchingStats {
                                    name,
                                    innings_pitched: stats_obj.get("inningsPitched").and_then(|ip| ip.as_str()).unwrap_or("0").to_string(),
                                    strikeouts: stats_obj.get("strikeOuts").and_then(|so| so.as_u64()).unwrap_or(0) as u32,
                                    earned_runs: stats_obj.get("earnedRuns").and_then(|er| er.as_u64()).unwrap_or(0) as u32,
                                    hits_allowed: stats_obj.get("hits").and_then(|h| h.as_u64()).unwrap_or(0) as u32,
                                    runs_allowed: stats_obj.get("runs").and_then(|r| r.as_u64()).unwrap_or(0) as u32,
                                    walks: stats_obj.get("baseOnBalls").and_then(|bb| bb.as_u64()).unwrap_or(0) as u32,
                                    home_runs_allowed: stats_obj.get("homeRuns").and_then(|hr| hr.as_u64()).unwrap_or(0) as u32,
                                    era: stats_obj.get("era").and_then(|era| era.as_str()).map(String::from),
                                });
                            }
                        }
                    }
                }
            }
            pitcher_stats
        } else {
            println!("DEBUG: No pitchers array found in team data");
            Vec::new()
        };
        
        // Print a debug message to show how many player stats we found
        println!("DEBUG: Found {} batters and {} pitchers", batters.len(), pitchers.len());
        
        Ok(TeamStats {
            team_name,
            batting,
            pitching,
            batters,
            pitchers,
        })
    }
}

impl Default for BattingStats {
    fn default() -> Self {
        Self {
            runs: 0,
            hits: 0,
            home_runs: 0,
            rbi: 0,
            stolen_bases: 0,
            avg: ".000".to_string(),
            obp: ".000".to_string(),
            slg: ".000".to_string(),
            ops: ".000".to_string(),
        }
    }
}

impl Default for PitchingStats {
    fn default() -> Self {
        Self {
            innings_pitched: "0.0".to_string(),
            hits_allowed: 0,
            runs_allowed: 0,
            earned_runs: 0,
            walks: 0,
            strikeouts: 0,
            home_runs_allowed: 0,
            era: "0.00".to_string(),
        }
    }
}

impl fmt::Display for GameStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AWAY: {}", self.away_team_stats)?;
        writeln!(f, "HOME: {}", self.home_team_stats)?;
        Ok(())
    }
}

impl fmt::Display for TeamStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.team_name)?;
        writeln!(f, "  BATTING: {}", self.batting)?;
        writeln!(f, "  PITCHING: {}", self.pitching)?;
        
        if !self.batters.is_empty() {
            writeln!(f, "  BATTERS:")?;
            // Print header
            writeln!(f, "    {:<25} {:<7} {:<3} {:<3} {:<3} {:<3} {:<5} {:<5} {:<5}", 
                "NAME", "AB", "H", "R", "HR", "RBI", "AVG", "OBP", "SLG")?;
            writeln!(f, "    {}", "-".repeat(70))?;
            
            for batter in &self.batters {
                writeln!(f, "    {}", batter)?;
            }
        }
        
        if !self.pitchers.is_empty() {
            writeln!(f, "  PITCHERS:")?;
            // Print header
            writeln!(f, "    {:<25} {:<5} {:<3} {:<3} {:<3} {:<3} {:<3} {:<5}", 
                "NAME", "IP", "H", "R", "ER", "BB", "K", "ERA")?;
            writeln!(f, "    {}", "-".repeat(60))?;
            
            for pitcher in &self.pitchers {
                writeln!(f, "    {}", pitcher)?;
            }
        }
        
        Ok(())
    }
}

impl fmt::Display for BattingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R: {}, H: {}, HR: {}, RBI: {}, SB: {}, AVG: {}, OBP: {}, SLG: {}, OPS: {}",
            self.runs, self.hits, self.home_runs, self.rbi, self.stolen_bases,
            self.avg, self.obp, self.slg, self.ops
        )
    }
}

impl fmt::Display for PitchingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IP: {}, H: {}, R: {}, ER: {}, BB: {}, SO: {}, HR: {}, ERA: {}",
            self.innings_pitched, self.hits_allowed, self.runs_allowed, self.earned_runs,
            self.walks, self.strikeouts, self.home_runs_allowed, self.era
        )
    }
}

impl fmt::Display for PlayerBattingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use provided AVG or calculate it
        let avg = if let Some(ref avg) = self.avg {
            avg.clone()
        } else if self.at_bats > 0 {
            format!(".{:03}", (self.hits as f32 / self.at_bats as f32 * 1000.0).round() as u32)
                .replace(".000", "---")
        } else {
            "---".to_string()
        };
        
        // Use provided OBP or use placeholder
        let obp = self.obp.clone().unwrap_or_else(|| "---".to_string());
        
        // Use provided SLG or use placeholder
        let slg = self.slg.clone().unwrap_or_else(|| "---".to_string());
        
        write!(
            f,
            "{:<25} {:<7} {:<3} {:<3} {:<3} {:<3} {:<5} {:<5} {:<5}",
            truncate_name(&self.name, 25),
            self.at_bats,
            self.hits,
            self.runs,
            self.home_runs,
            self.rbi,
            avg,
            obp,
            slg
        )
    }
}

impl fmt::Display for PlayerPitchingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use provided ERA or calculate it
        let era = if let Some(ref era) = self.era {
            era.clone()
        } else {
            let ip_float = parse_innings_pitched(&self.innings_pitched);
            if ip_float > 0.0 {
                format!("{:.2}", (self.earned_runs as f32 * 9.0) / ip_float)
            } else {
                "-.--".to_string()
            }
        };
        
        write!(
            f,
            "{:<25} {:<5} {:<3} {:<3} {:<3} {:<3} {:<3} {:<5}",
            truncate_name(&self.name, 25),
            self.innings_pitched,
            self.hits_allowed,
            self.runs_allowed,
            self.earned_runs,
            self.walks,
            self.strikeouts,
            era
        )
    }
}

// Helper function to truncate player names to fit in display
fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[0..max_len-3])
    }
}

// Helper function to parse innings pitched string to float
fn parse_innings_pitched(ip: &str) -> f32 {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 2 {
        let innings = parts[0].parse::<f32>().unwrap_or(0.0);
        let fraction = parts[1].parse::<f32>().unwrap_or(0.0) / 3.0;
        innings + fraction
    } else {
        ip.parse::<f32>().unwrap_or(0.0)
    }
}

impl fmt::Display for GameInnings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Game: {} vs {}", self.home_team.name, self.away_team.name)?;
        writeln!(f, "Date: {}", self.game_date.split('T').next().unwrap_or(&self.game_date))?;
        writeln!(f, "Status: {}", self.status)?;
        
        // Create a table header for innings
        write!(f, "     ")?;
        for i in 0..self.innings.len() {
            write!(f, " {:2}", i + 1)?;
        }
        writeln!(f, "  | R")?;
        
        // Add a separator line
        writeln!(f, "-----{}--+--", "-".repeat(self.innings.len() * 3))?;
        
        // Away team line
        write!(f, "{:<3} |", "Awa")?;
        for inning in &self.innings {
            if let Some(runs) = inning.away {
                write!(f, " {:2}", runs)?;
            } else {
                write!(f, "  -")?;
            }
        }
        writeln!(f, "  | {}", self.away_runs.unwrap_or(0))?;
        
        // Home team line
        write!(f, "{:<3} |", "Hom")?;
        for inning in &self.innings {
            if let Some(runs) = inning.home {
                write!(f, " {:2}", runs)?;
            } else {
                write!(f, "  -")?;
            }
        }
        writeln!(f, "  | {}", self.home_runs.unwrap_or(0))?;
        
        Ok(())
    }
}

// Create a default instance for convenience using OnceLock instead of static mut
static MLB_API_INSTANCE: OnceLock<MlbApi> = OnceLock::new();

/// Initialize the MLB API instance if it hasn't been initialized yet
fn get_mlb_api() -> &'static MlbApi {
    MLB_API_INSTANCE.get_or_init(|| MlbApi::new())
}

/// Get player information by ID
pub async fn get_player(player_id: u32) -> Result<Player> {
    get_mlb_api().get_player(player_id).await
}

/// Get team information by ID
pub async fn get_team(team_id: u32) -> Result<Team> {
    get_mlb_api().get_team(team_id).await
}

/// Get team schedule by ID
pub async fn get_team_schedule(team_id: u32, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<Game>> {
    get_mlb_api().get_team_schedule(team_id, start_date, end_date).await
}

/// Get game information by ID
pub async fn get_game(game_id: u64) -> Result<Game> {
    get_mlb_api().get_game(game_id).await
}

/// Get all MLB games for today
pub async fn get_todays_games() -> Result<Vec<Game>> {
    get_mlb_api().get_todays_games().await
}

/// Get all MLB games from yesterday
pub async fn get_yesterdays_games() -> Result<Vec<Game>> {
    get_mlb_api().get_yesterdays_games().await
}

/// Get detailed game statistics by ID
pub async fn get_game_stats(game_id: u32) -> Result<GameStats> {
    get_mlb_api().get_game_stats(game_id).await
}

/// Get inning-by-inning breakdown for a game
pub async fn get_game_innings(game_id: u32) -> Result<GameInnings> {
    get_mlb_api().get_game_innings(game_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_player() -> Player {
        Player {
            id: 1,
            full_name: "Test Player".to_string(),
            first_name: "Test".to_string(),
            last_name: "Player".to_string(),
            primary_number: Some("42".to_string()),
            current_team: Some(Team {
                id: 1,
                name: "Test Team".to_string(),
                team_code: Some("tst".to_string()),
                file_code: Some("tst".to_string()),
                team_name: Some("Test".to_string()),
                location_name: Some("Test City".to_string()),
                short_name: Some("Test".to_string()),
                abbreviation: Some("TST".to_string()),
                franchise_name: Some("Test".to_string()),
                club_name: Some("Test".to_string()),
                first_year_of_play: Some("2024".to_string()),
                active: Some(true),
                venue: Some(Venue {
                    id: 1,
                    name: "Test Venue".to_string(),
                }),
                league: Some(League {
                    id: 1,
                    name: "Test League".to_string(),
                }),
                division: Some(Division {
                    id: 1,
                    name: "Test Division".to_string(),
                }),
            }),
            position: Some(Position {
                code: "P".to_string(),
                name: "Pitcher".to_string(),
                position_type: "Pitcher".to_string(),
                abbreviation: "P".to_string(),
            }),
            bat_side: Some(BatSide {
                code: "R".to_string(),
                description: "Right".to_string(),
            }),
            pitch_hand: Some(PitchHand {
                code: "R".to_string(),
                description: "Right".to_string(),
            }),
            birth_date: None,
            birth_city: None,
            birth_country: None,
            height: None,
            weight: None,
            current_age: None,
            mlb_debut_date: None,
            nick_name: None,
            active: Some(true),
        }
    }

    fn mock_team() -> Team {
        Team {
            id: 1,
            name: "Test Team".to_string(),
            team_code: Some("tst".to_string()),
            file_code: Some("tst".to_string()),
            team_name: Some("Test".to_string()),
            location_name: Some("Test City".to_string()),
            short_name: Some("Test".to_string()),
            abbreviation: Some("TST".to_string()),
            franchise_name: Some("Test".to_string()),
            club_name: Some("Test".to_string()),
            first_year_of_play: Some("2024".to_string()),
            active: Some(true),
            venue: Some(Venue {
                id: 1,
                name: "Test Venue".to_string(),
            }),
            league: Some(League {
                id: 1,
                name: "Test League".to_string(),
            }),
            division: Some(Division {
                id: 1,
                name: "Test Division".to_string(),
            }),
        }
    }

    fn mock_game() -> Game {
        Game {
            game_pk: 12345,
            game_date: "2024-03-28T13:05:00Z".to_string(),
            status: GameState::Final,
            teams: GameTeams {
                away: GameTeam {
                    score: Some(3),
                    team: Team {
                        id: 1,
                        name: "White Sox".to_string(),
                        team_code: Some("cws".to_string()),
                        file_code: Some("cws".to_string()),
                        team_name: Some("White Sox".to_string()),
                        location_name: Some("Chicago".to_string()),
                        short_name: Some("White Sox".to_string()),
                        abbreviation: Some("CWS".to_string()),
                        franchise_name: None,
                        club_name: None,
                        first_year_of_play: None,
                        active: None,
                        venue: None,
                        league: None,
                        division: None,
                    },
                    is_winner: Some(false),
                },
                home: GameTeam {
                    score: Some(5),
                    team: Team {
                        id: 2,
                        name: "Cubs".to_string(),
                        team_code: Some("chc".to_string()),
                        file_code: Some("chc".to_string()),
                        team_name: Some("Cubs".to_string()),
                        location_name: Some("Chicago".to_string()),
                        short_name: Some("Cubs".to_string()),
                        abbreviation: Some("CHC".to_string()),
                        franchise_name: None,
                        club_name: None,
                        first_year_of_play: None,
                        active: None,
                        venue: None,
                        league: None,
                        division: None,
                    },
                    is_winner: Some(true),
                },
            },
            venue: Some(Venue {
                id: 1,
                name: "Wrigley Field".to_string(),
            }),
        }
    }

    #[test]
    fn test_get_player() {
        let player = mock_player();
        let display = format!("{}", player);
        assert!(display.contains("Test Player"));
        assert!(display.contains("Pitcher"));
    }

    #[test]
    fn test_get_team() {
        let team = mock_team();
        let display = format!("{}", team);
        assert!(display.contains("Test Team"));
        assert!(display.contains("TST"));
        assert!(display.contains("Test League"));
        assert!(display.contains("Test Venue"));
    }

    #[test]
    fn test_get_team_schedule() {
        // Create a mock game
        let game = mock_game();
        
        // Test the display of the game
        let display = format!("{}", game);
        
        assert!(display.contains("Chicago White Sox"));
        assert!(display.contains("Chicago Cubs"));
    }
    
    #[test]
    fn test_get_game() {
        // Create a mock game
        let game = mock_game();
        
        // Test the display of the game
        let display = format!("{}", game);
        
        assert!(display.contains("Chicago White Sox"));
        assert!(display.contains("Chicago Cubs"));
    }

    #[test]
    fn test_game_stats_display() {
        // Create mock game stats
        let game_stats = GameStats {
            away_team_stats: TeamStats {
                team_name: "Test Away Team".to_string(),
                batting: BattingStats {
                    runs: 3,
                    hits: 8,
                    home_runs: 1,
                    rbi: 3,
                    stolen_bases: 2,
                    avg: ".267".to_string(),
                    obp: ".333".to_string(),
                    slg: ".400".to_string(),
                    ops: ".733".to_string(),
                },
                pitching: PitchingStats {
                    innings_pitched: "9.0".to_string(),
                    hits_allowed: 6,
                    runs_allowed: 2,
                    earned_runs: 2,
                    walks: 3,
                    strikeouts: 10,
                    home_runs_allowed: 0,
                    era: "2.00".to_string(),
                },
                batters: vec![
                    PlayerBattingStats {
                        name: "Test Batter 1".to_string(),
                        hits: 3,
                        at_bats: 4,
                        home_runs: 1,
                        rbi: 2,
                        runs: 1,
                        doubles: 1,
                        triples: 0,
                        stolen_bases: 1,
                        walks: 0,
                        strikeouts: 1,
                        avg: Some(".750".to_string()),
                        obp: Some(".750".to_string()),
                        slg: Some("1.500".to_string()),
                    },
                    PlayerBattingStats {
                        name: "Test Batter 2".to_string(),
                        hits: 2,
                        at_bats: 4,
                        home_runs: 0,
                        rbi: 1,
                        runs: 1,
                        doubles: 1,
                        triples: 0,
                        stolen_bases: 0,
                        walks: 0,
                        strikeouts: 1,
                        avg: Some(".500".to_string()),
                        obp: Some(".500".to_string()),
                        slg: Some(".750".to_string()),
                    },
                    PlayerBattingStats {
                        name: "Test Batter 3".to_string(),
                        hits: 1,
                        at_bats: 3,
                        home_runs: 0,
                        rbi: 0,
                        runs: 1,
                        doubles: 0,
                        triples: 0,
                        stolen_bases: 1,
                        walks: 1,
                        strikeouts: 0,
                        avg: Some(".333".to_string()),
                        obp: Some(".500".to_string()),
                        slg: Some(".333".to_string()),
                    },
                ],
                pitchers: vec![
                    PlayerPitchingStats {
                        name: "Test Pitcher 1".to_string(),
                        innings_pitched: "6.0".to_string(),
                        strikeouts: 7,
                        earned_runs: 1,
                        hits_allowed: 4,
                        runs_allowed: 1,
                        walks: 2,
                        home_runs_allowed: 0,
                        era: Some("1.50".to_string()),
                    },
                    PlayerPitchingStats {
                        name: "Test Pitcher 2".to_string(),
                        innings_pitched: "3.0".to_string(),
                        strikeouts: 3,
                        earned_runs: 1,
                        hits_allowed: 2,
                        runs_allowed: 1,
                        walks: 1,
                        home_runs_allowed: 0,
                        era: Some("3.00".to_string()),
                    },
                ],
            },
            home_team_stats: TeamStats {
                team_name: "Test Home Team".to_string(),
                batting: BattingStats {
                    runs: 2,
                    hits: 6,
                    home_runs: 0,
                    rbi: 2,
                    stolen_bases: 1,
                    avg: ".222".to_string(),
                    obp: ".300".to_string(),
                    slg: ".333".to_string(),
                    ops: ".633".to_string(),
                },
                pitching: PitchingStats {
                    innings_pitched: "9.0".to_string(),
                    hits_allowed: 8,
                    runs_allowed: 3,
                    earned_runs: 3,
                    walks: 2,
                    strikeouts: 8,
                    home_runs_allowed: 1,
                    era: "3.00".to_string(),
                },
                batters: vec![
                    PlayerBattingStats {
                        name: "Test Batter 3".to_string(),
                        hits: 2,
                        at_bats: 4,
                        home_runs: 0,
                        rbi: 1,
                        runs: 1,
                        doubles: 1,
                        triples: 0,
                        stolen_bases: 0,
                        walks: 0,
                        strikeouts: 1,
                        avg: Some(".500".to_string()),
                        obp: Some(".500".to_string()),
                        slg: Some(".750".to_string()),
                    },
                    PlayerBattingStats {
                        name: "Test Batter 4".to_string(),
                        hits: 1,
                        at_bats: 3,
                        home_runs: 0,
                        rbi: 1,
                        runs: 0,
                        doubles: 0,
                        triples: 0,
                        stolen_bases: 1,
                        walks: 1,
                        strikeouts: 1,
                        avg: Some(".333".to_string()),
                        obp: Some(".500".to_string()),
                        slg: Some(".333".to_string()),
                    },
                ],
                pitchers: vec![
                    PlayerPitchingStats {
                        name: "Test Pitcher 1".to_string(),
                        innings_pitched: "6.0".to_string(),
                        strikeouts: 7,
                        earned_runs: 1,
                        hits_allowed: 4,
                        runs_allowed: 1,
                        walks: 2,
                        home_runs_allowed: 0,
                        era: Some("1.50".to_string()),
                    },
                    PlayerPitchingStats {
                        name: "Test Pitcher 2".to_string(),
                        innings_pitched: "3.0".to_string(),
                        strikeouts: 3,
                        earned_runs: 1,
                        hits_allowed: 2,
                        runs_allowed: 1,
                        walks: 1,
                        home_runs_allowed: 0,
                        era: Some("3.00".to_string()),
                    },
                ],
            },
        };
        
        let display = format!("{}", game_stats);
        
        // Verify the display output contains expected information
        assert!(display.contains("Test Away Team"));
        assert!(display.contains("Test Home Team"));
        assert!(display.contains("R: 3, H: 8, HR: 1"));
        assert!(display.contains("IP: 9.0, H: 6, R: 2"));
        assert!(display.contains("BATTERS:"));
        assert!(display.contains("PITCHERS:"));
        assert!(display.contains("Test Batter 1"));
        assert!(display.contains("Test Pitcher 1"));
        assert!(display.contains("Test Batter 3"));
        assert!(display.contains("Test Pitcher 2"));
    }

    #[test]
    fn test_game_innings_display() {
        // Create mock game innings
        let game_innings = GameInnings {
            game_pk: 1,
            game_date: "2024-03-28T13:05:00Z".to_string(),
            status: GameState::Final,
            home_team: Team {
                id: 1,
                name: "Home Team".to_string(),
                team_code: None,
                file_code: None,
                team_name: None,
                location_name: None,
                short_name: None,
                abbreviation: None,
                franchise_name: None,
                club_name: None,
                first_year_of_play: None,
                active: None,
                venue: None,
                league: None,
                division: None,
            },
            away_team: Team {
                id: 2,
                name: "Away Team".to_string(),
                team_code: None,
                file_code: None,
                team_name: None,
                location_name: None,
                short_name: None,
                abbreviation: None,
                franchise_name: None,
                club_name: None,
                first_year_of_play: None,
                active: None,
                venue: None,
                league: None,
                division: None,
            },
            innings: vec![
                InningData {
                    inning: 1,
                    home: Some(1),
                    away: Some(0),
                },
                InningData {
                    inning: 2,
                    home: Some(0),
                    away: Some(2),
                },
            ],
            home_runs: Some(1),
            away_runs: Some(2),
        };

        let display = format!("{}", game_innings);
        
        // Verify the display output contains expected information
        assert!(display.contains("Game: Home Team vs Away Team"));
        assert!(display.contains("Date: 2024-03-28"));
        assert!(display.contains("Status: Final"));
        assert!(display.contains("Awa |  0  2  | 2"));
        assert!(display.contains("Hom |  1  0  | 1"));
    }

    #[test]
    fn test_get_todays_games() {
        // Create a mock game
        let game = mock_game();
        
        // Create a mock schedule with the game
        let schedule_date = ScheduleDate {
            date: "2024-03-28".to_string(),
            games: vec![game],
        };
        
        let schedule = Schedule {
            dates: vec![schedule_date],
        };
        
        // Convert the schedule to JSON
        let _schedule_json = serde_json::to_string(&schedule).unwrap();
        
        // Create a mock API instance
        let _api = MlbApi::new();
        
        // Test the display of the game
        let game = mock_game();
        let display = format!("{}", game);
        
        assert!(display.contains("Chicago White Sox"));
        assert!(display.contains("Chicago Cubs"));
        assert!(display.contains("Date: 2024-03-28"));
    }
    
    #[test]
    fn test_get_yesterdays_games() {
        // Create a mock game
        let game = mock_game();
        
        // Create a mock schedule with the game
        let schedule_date = ScheduleDate {
            date: "2024-03-27".to_string(),
            games: vec![game],
        };
        
        let schedule = Schedule {
            dates: vec![schedule_date],
        };
        
        // Convert the schedule to JSON
        let _schedule_json = serde_json::to_string(&schedule).unwrap();
        
        // Create a mock API instance
        let _api = MlbApi::new();
        
        // Test the display of the game
        let game = mock_game();
        let display = format!("{}", game);
        
        assert!(display.contains("Chicago White Sox"));
        assert!(display.contains("Chicago Cubs"));
        assert!(display.contains("Date: 2024-03-28"));
    }
} 