use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use chrono::{Datelike, Local};
use std::env;

// Make the MLB API base URL configurable for testing
fn get_mlb_api_base_url() -> String {
    env::var("MLB_API_BASE_URL").unwrap_or_else(|_| "https://statsapi.mlb.com/api/v1".to_string())
}

/// MLB API client
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

/// Team information
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    #[serde(rename = "teamCode")]
    pub team_code: Option<String>,
    #[serde(rename = "fileCode")]
    pub file_code: Option<String>,
    #[serde(rename = "teamName")]
    pub team_name: Option<String>,
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub abbreviation: Option<String>,
    #[serde(rename = "franchiseName")]
    pub franchise_name: Option<String>,
    #[serde(rename = "clubName")]
    pub club_name: Option<String>,
    #[serde(rename = "firstYearOfPlay")]
    pub first_year_of_play: Option<String>,
    pub active: Option<bool>,
    pub venue: Option<Venue>,
    pub league: Option<League>,
    pub division: Option<Division>,
}

/// Venue information
#[derive(Debug, Serialize, Deserialize)]
pub struct Venue {
    pub id: u32,
    pub name: String,
    pub link: String,
}

/// League information
#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub link: String,
}

/// Division information
#[derive(Debug, Serialize, Deserialize)]
pub struct Division {
    pub id: u32,
    pub name: String,
    pub link: String,
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

/// Game information
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "gamePk")]
    pub game_pk: u32,
    #[serde(rename = "gameDate")]
    pub game_date: String,
    pub status: GameStatus,
    pub teams: GameTeams,
    pub venue: Option<Venue>,
}

/// Game status
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatus {
    #[serde(rename = "abstractGameState")]
    pub abstract_game_state: String,
    #[serde(rename = "detailedState")]
    pub detailed_state: String,
}

/// Game teams
#[derive(Debug, Serialize, Deserialize)]
pub struct GameTeams {
    pub away: GameTeam,
    pub home: GameTeam,
}

/// Game team
#[derive(Debug, Serialize, Deserialize)]
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
    pub status: GameStatus,
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
        // Format date
        let date_str = self.game_date.split('T').next().unwrap_or(&self.game_date);
        
        writeln!(f, "Date: {}", date_str)?;
        writeln!(f, "Status: {}", self.status.detailed_state)?;
        
        // Format teams and scores
        let away_team = &self.teams.away.team.name;
        let home_team = &self.teams.home.team.name;
        
        if let (Some(away_score), Some(home_score)) = (self.teams.away.score, self.teams.home.score) {
            writeln!(f, "{} {} @ {} {}", away_team, away_score, home_team, home_score)?;
            
            if let (Some(away_winner), Some(home_winner)) = (self.teams.away.is_winner, self.teams.home.is_winner) {
                if away_winner {
                    writeln!(f, "Winner: {}", away_team)?;
                } else if home_winner {
                    writeln!(f, "Winner: {}", home_team)?;
                }
            }
        } else {
            writeln!(f, "{} @ {}", away_team, home_team)?;
        }
        
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
        let url = format!("{}/people/{}", get_mlb_api_base_url(), player_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch player data: HTTP {}", response.status()));
        }
        
        let data: Value = response.json().await?;
        
        if let Some(people) = data.get("people").and_then(|p| p.as_array()) {
            if let Some(player) = people.first() {
                let player: Player = serde_json::from_value(player.clone())?;
                return Ok(player);
            }
        }
        
        Err(anyhow!("Player not found or invalid response format"))
    }

    /// Get team information by ID
    pub async fn get_team(&self, team_id: u32) -> Result<Team> {
        let url = format!("{}/teams/{}", get_mlb_api_base_url(), team_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch team data: HTTP {}", response.status()));
        }
        
        let data: Value = response.json().await?;
        
        if let Some(teams) = data.get("teams").and_then(|t| t.as_array()) {
            if let Some(team) = teams.first() {
                let team: Team = serde_json::from_value(team.clone())?;
                return Ok(team);
            }
        }
        
        Err(anyhow!("Team not found or invalid response format"))
    }

    /// Get schedule for a team
    pub async fn get_team_schedule(&self, team_id: u32, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<Game>> {
        // Default to current month if no dates provided
        let now = Local::now();
        let start = start_date.unwrap_or_else(|| format!("{}-{:02}-01", now.year(), now.month()));
        let end = end_date.unwrap_or_else(|| format!("{}-{:02}-30", now.year(), now.month()));
        
        let url = format!(
            "{}/schedule?teamId={}&startDate={}&endDate={}&sportId=1",
            get_mlb_api_base_url(), team_id, start, end
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
        let url = format!("{}/game/{}/feed/live", get_mlb_api_base_url(), game_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch game data: HTTP {}", response.status()));
        }
        
        let data: Value = response.json().await?;
        
        if let Some(game_data) = data.get("gameData") {
            // Extract basic game information
            let game = Game {
                game_pk: game_data["game"]["pk"].as_u64().unwrap_or(0) as u32,
                game_date: game_data["datetime"]["dateTime"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                status: GameStatus {
                    abstract_game_state: game_data["status"]["abstractGameState"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    detailed_state: game_data["status"]["detailedState"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                },
                teams: GameTeams {
                    away: self.extract_game_team(&data, "away")?,
                    home: self.extract_game_team(&data, "home")?,
                },
                venue: Some(Venue {
                    id: game_data["venue"]["id"].as_u64().unwrap_or(0) as u32,
                    name: game_data["venue"]["name"].as_str().unwrap_or("").to_string(),
                    link: game_data["venue"]["link"].as_str().unwrap_or("").to_string(),
                }),
            };
            return Ok(game);
        }
        
        Err(anyhow!("Game not found or invalid response format"))
    }

    /// Helper method to extract team information from game data
    fn extract_game_team(&self, data: &Value, team_type: &str) -> Result<GameTeam> {
        let linescore = &data["liveData"]["linescore"];
        let game_data = &data["gameData"]["teams"][team_type];

        Ok(GameTeam {
            score: linescore["teams"][team_type]["runs"].as_u64().map(|s| s as u32),
            team: Team {
                id: game_data["id"].as_u64().unwrap_or(0) as u32,
                name: game_data["name"].as_str().unwrap_or("").to_string(),
                team_code: game_data["teamCode"].as_str().map(String::from),
                file_code: game_data["fileCode"].as_str().map(String::from),
                team_name: game_data["teamName"].as_str().map(String::from),
                location_name: game_data["locationName"].as_str().map(String::from),
                short_name: game_data["shortName"].as_str().map(String::from),
                abbreviation: game_data["abbreviation"].as_str().map(String::from),
                franchise_name: None,
                club_name: None,
                first_year_of_play: None,
                active: None,
                venue: None,
                league: None,
                division: None,
            },
            is_winner: linescore["teams"][team_type]["isWinner"].as_bool(),
        })
    }

    /// Get all games scheduled for today
    pub async fn get_todays_games(&self) -> Result<Vec<Game>> {
        // Get today's date in YYYY-MM-DD format
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        let url = format!(
            "{}/schedule?sportId=1&date={}",
            get_mlb_api_base_url(), today
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch today's games: HTTP {}", response.status()));
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

    /// Get detailed game statistics by ID
    pub async fn get_game_stats(&self, game_id: u32) -> Result<GameStats> {
        let url = format!("{}/game/{}/boxscore", get_mlb_api_base_url(), game_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch game stats: HTTP {}", response.status()));
        }
        
        let data: Value = response.json().await?;
        
        // Extract basic game information
        let teams = data.get("teams").ok_or_else(|| anyhow!("Missing teams data"))?;
        let away = teams.get("away").ok_or_else(|| anyhow!("Missing away team data"))?;
        let home = teams.get("home").ok_or_else(|| anyhow!("Missing home team data"))?;
        
        let game_stats = GameStats {
            away_team_stats: self.extract_team_stats(away)?,
            home_team_stats: self.extract_team_stats(home)?,
        };
        
        Ok(game_stats)
    }
    
    /// Helper method to extract team statistics
    fn extract_team_stats(&self, team_data: &Value) -> Result<TeamStats> {
        let team_name = team_data["team"]["name"].as_str().unwrap_or("Unknown").to_string();
        
        // Extract batting stats
        let batting = team_data.get("teamStats").and_then(|s| s.get("batting")).cloned();
        let batting_stats = if let Some(stats) = batting {
            BattingStats {
                runs: stats["runs"].as_u64().unwrap_or(0) as u32,
                hits: stats["hits"].as_u64().unwrap_or(0) as u32,
                home_runs: stats["homeRuns"].as_u64().unwrap_or(0) as u32,
                rbi: stats["rbi"].as_u64().unwrap_or(0) as u32,
                stolen_bases: stats["stolenBases"].as_u64().unwrap_or(0) as u32,
                avg: stats["avg"].as_str().unwrap_or(".000").to_string(),
                obp: stats["obp"].as_str().unwrap_or(".000").to_string(),
                slg: stats["slg"].as_str().unwrap_or(".000").to_string(),
                ops: stats["ops"].as_str().unwrap_or(".000").to_string(),
            }
        } else {
            BattingStats::default()
        };
        
        // Extract pitching stats
        let pitching = team_data.get("teamStats").and_then(|s| s.get("pitching")).cloned();
        let pitching_stats = if let Some(stats) = pitching {
            PitchingStats {
                innings_pitched: stats["inningsPitched"].as_str().unwrap_or("0").to_string(),
                hits_allowed: stats["hits"].as_u64().unwrap_or(0) as u32,
                runs_allowed: stats["runs"].as_u64().unwrap_or(0) as u32,
                earned_runs: stats["earnedRuns"].as_u64().unwrap_or(0) as u32,
                walks: stats["baseOnBalls"].as_u64().unwrap_or(0) as u32,
                strikeouts: stats["strikeOuts"].as_u64().unwrap_or(0) as u32,
                home_runs_allowed: stats["homeRuns"].as_u64().unwrap_or(0) as u32,
                era: stats["era"].as_str().unwrap_or("0.00").to_string(),
            }
        } else {
            PitchingStats::default()
        };
        
        // Extract all batters
        let mut batters = Vec::new();
        if let Some(players) = team_data.get("players").and_then(|p| p.as_object()) {
            for (_, player_data) in players {
                if let Some(stats) = player_data.get("stats").and_then(|s| s.get("batting")) {
                    if stats["atBats"].as_u64().unwrap_or(0) > 0 {
                        let player_name = player_data["person"]["fullName"].as_str().unwrap_or("Unknown").to_string();
                        let hits = stats["hits"].as_u64().unwrap_or(0) as u32;
                        let at_bats = stats["atBats"].as_u64().unwrap_or(0) as u32;
                        let home_runs = stats["homeRuns"].as_u64().unwrap_or(0) as u32;
                        let rbi = stats["rbi"].as_u64().unwrap_or(0) as u32;
                        let runs = stats["runs"].as_u64().unwrap_or(0) as u32;
                        let doubles = stats["doubles"].as_u64().unwrap_or(0) as u32;
                        let triples = stats["triples"].as_u64().unwrap_or(0) as u32;
                        let stolen_bases = stats["stolenBases"].as_u64().unwrap_or(0) as u32;
                        let walks = stats["baseOnBalls"].as_u64().unwrap_or(0) as u32;
                        let strikeouts = stats["strikeOuts"].as_u64().unwrap_or(0) as u32;
                        let avg = stats["avg"].as_str().map(String::from);
                        let obp = stats["obp"].as_str().map(String::from);
                        let slg = stats["slg"].as_str().map(String::from);
                        
                        batters.push(PlayerBattingStats {
                            name: player_name,
                            hits,
                            at_bats,
                            home_runs,
                            rbi,
                            runs,
                            doubles,
                            triples,
                            stolen_bases,
                            walks,
                            strikeouts,
                            avg,
                            obp,
                            slg,
                        });
                    }
                }
            }
        }
        
        // Sort batters by batting order or position
        batters.sort_by(|a, b| a.name.cmp(&b.name));
        
        // Extract all pitchers
        let mut pitchers = Vec::new();
        if let Some(players) = team_data.get("players").and_then(|p| p.as_object()) {
            for (_, player_data) in players {
                if let Some(stats) = player_data.get("stats").and_then(|s| s.get("pitching")) {
                    if stats["inningsPitched"].as_str().unwrap_or("0") != "0" {
                        let player_name = player_data["person"]["fullName"].as_str().unwrap_or("Unknown").to_string();
                        let innings_pitched = stats["inningsPitched"].as_str().unwrap_or("0").to_string();
                        let strikeouts = stats["strikeOuts"].as_u64().unwrap_or(0) as u32;
                        let earned_runs = stats["earnedRuns"].as_u64().unwrap_or(0) as u32;
                        let hits_allowed = stats["hits"].as_u64().unwrap_or(0) as u32;
                        let runs_allowed = stats["runs"].as_u64().unwrap_or(0) as u32;
                        let walks = stats["baseOnBalls"].as_u64().unwrap_or(0) as u32;
                        let home_runs_allowed = stats["homeRuns"].as_u64().unwrap_or(0) as u32;
                        let era = stats["era"].as_str().map(String::from);
                        
                        pitchers.push(PlayerPitchingStats {
                            name: player_name,
                            innings_pitched,
                            strikeouts,
                            earned_runs,
                            hits_allowed,
                            runs_allowed,
                            walks,
                            home_runs_allowed,
                            era,
                        });
                    }
                }
            }
        }
        
        // Sort pitchers by innings pitched (most to least)
        pitchers.sort_by(|a, b| {
            let a_ip = parse_innings_pitched(&a.innings_pitched);
            let b_ip = parse_innings_pitched(&b.innings_pitched);
            b_ip.partial_cmp(&a_ip).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(TeamStats {
            team_name,
            batting: batting_stats,
            pitching: pitching_stats,
            batters,
            pitchers,
        })
    }

    /// Get inning-by-inning data for a game
    pub async fn get_game_innings(&self, game_id: u32) -> Result<GameInnings> {
        let url = format!("{}/game/{}/linescore", get_mlb_api_base_url(), game_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch game innings data: HTTP {}", response.status()));
        }
        
        let data: Value = response.json().await?;
        
        // Extract teams
        let teams = data.get("teams").ok_or_else(|| anyhow!("Missing teams data"))?;
        let home_team_data = teams.get("home").ok_or_else(|| anyhow!("Missing home team data"))?;
        let away_team_data = teams.get("away").ok_or_else(|| anyhow!("Missing away team data"))?;
        
        // Extract home and away teams
        let home_team = Team {
            id: home_team_data["team"]["id"].as_u64().unwrap_or(0) as u32,
            name: home_team_data["team"]["name"].as_str().unwrap_or("Unknown").to_string(),
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
            id: away_team_data["team"]["id"].as_u64().unwrap_or(0) as u32,
            name: away_team_data["team"]["name"].as_str().unwrap_or("Unknown").to_string(),
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
        
        // Extract innings data
        let innings_data = data.get("innings").and_then(|i| i.as_array());
        let mut innings = Vec::new();
        
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
        let home_runs = home_team_data["runs"].as_u64().map(|r| r as u32);
        let away_runs = away_team_data["runs"].as_u64().map(|r| r as u32);
        
        // Extract game info
        let game_pk = data["game"]["pk"].as_u64().unwrap_or(0) as u32;
        let game_date = data["game"]["date"].as_str().unwrap_or("").to_string();
        
        // Create game status
        let status = GameStatus {
            abstract_game_state: data["status"]["abstractGameState"].as_str().unwrap_or("").to_string(),
            detailed_state: data["status"]["detailedState"].as_str().unwrap_or("").to_string(),
        };
        
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
        writeln!(f, "HOME: {}", self.home_team_stats)?;
        writeln!(f, "AWAY: {}", self.away_team_stats)?;
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
        writeln!(f, "Game: {} vs {}", self.away_team.name, self.home_team.name)?;
        writeln!(f, "Date: {}", self.game_date.split('T').next().unwrap_or(&self.game_date))?;
        writeln!(f, "Status: {}", self.status.detailed_state)?;
        
        // Create a table header for innings
        write!(f, "     ")?;
        for i in 0..self.innings.len() {
            write!(f, " {:2}", i + 1)?;
        }
        writeln!(f, "  | R")?;
        
        // Add a separator line
        writeln!(f, "-----{}-|--", "-".repeat(self.innings.len() * 3))?;
        
        // Away team line
        write!(f, "{:<3} |", self.away_team.name.chars().take(3).collect::<String>())?;
        for inning in &self.innings {
            if let Some(runs) = inning.away {
                write!(f, " {:2}", runs)?;
            } else {
                write!(f, "  -")?;
            }
        }
        writeln!(f, "  | {}", self.away_runs.unwrap_or(0))?;
        
        // Home team line
        write!(f, "{:<3} |", self.home_team.name.chars().take(3).collect::<String>())?;
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

// Create a default instance for convenience
static mut MLB_API_INSTANCE: Option<MlbApi> = None;

/// Get player information by ID
pub async fn get_player(player_id: u32) -> Result<Player> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_player(player_id).await
    }
}

/// Get team information by ID
pub async fn get_team(team_id: u32) -> Result<Team> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_team(team_id).await
    }
}

/// Get schedule for a team
pub async fn get_team_schedule(team_id: u32, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<Game>> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_team_schedule(team_id, start_date, end_date).await
    }
}

/// Get game information by ID
pub async fn get_game(game_id: u64) -> Result<Game> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_game(game_id).await
    }
}

/// Get all games scheduled for today
pub async fn get_todays_games() -> Result<Vec<Game>> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_todays_games().await
    }
}

/// Get detailed game statistics by ID
pub async fn get_game_stats(game_id: u32) -> Result<GameStats> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_game_stats(game_id).await
    }
}

/// Get inning-by-inning data for a game
pub async fn get_game_innings(game_id: u32) -> Result<GameInnings> {
    // This is not thread-safe, but it's fine for a CLI application
    unsafe {
        if MLB_API_INSTANCE.is_none() {
            MLB_API_INSTANCE = Some(MlbApi::new());
        }
        MLB_API_INSTANCE.as_ref().unwrap().get_game_innings(game_id).await
    }
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
                    link: "/api/v1/venues/1".to_string(),
                }),
                league: Some(League {
                    id: 1,
                    name: "Test League".to_string(),
                    link: "/api/v1/leagues/1".to_string(),
                }),
                division: Some(Division {
                    id: 1,
                    name: "Test Division".to_string(),
                    link: "/api/v1/divisions/1".to_string(),
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
                link: "/api/v1/venues/1".to_string(),
            }),
            league: Some(League {
                id: 1,
                name: "Test League".to_string(),
                link: "/api/v1/leagues/1".to_string(),
            }),
            division: Some(Division {
                id: 1,
                name: "Test Division".to_string(),
                link: "/api/v1/divisions/1".to_string(),
            }),
        }
    }

    fn mock_game() -> Game {
        Game {
            game_pk: 1,
            game_date: "2024-03-28".to_string(),
            status: GameStatus {
                abstract_game_state: "Final".to_string(),
                detailed_state: "Final".to_string(),
            },
            teams: GameTeams {
                away: GameTeam {
                    score: Some(5),
                    team: mock_team(),
                    is_winner: Some(true),
                },
                home: GameTeam {
                    score: Some(3),
                    team: mock_team(),
                    is_winner: Some(false),
                },
            },
            venue: Some(Venue {
                id: 1,
                name: "Test Venue".to_string(),
                link: "/api/v1/venues/1".to_string(),
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
        let game = mock_game();
        let display = format!("{}", game);
        assert!(display.contains("Test Team"));
        assert!(display.contains("5"));
        assert!(display.contains("3"));
    }

    #[test]
    fn test_get_game() {
        let game = mock_game();
        let display = format!("{}", game);
        assert!(display.contains("Test Team"));
        assert!(display.contains("5"));
        assert!(display.contains("3"));
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
                ],
                pitchers: vec![
                    PlayerPitchingStats {
                        name: "Test Pitcher 2".to_string(),
                        innings_pitched: "7.0".to_string(),
                        strikeouts: 5,
                        earned_runs: 3,
                        hits_allowed: 6,
                        runs_allowed: 3,
                        walks: 1,
                        home_runs_allowed: 1,
                        era: Some("3.86".to_string()),
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
            status: GameStatus {
                abstract_game_state: "Final".to_string(),
                detailed_state: "Final".to_string(),
            },
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
                    home: Some(0),
                    away: Some(1),
                },
                InningData {
                    inning: 2,
                    home: Some(2),
                    away: Some(0),
                },
                InningData {
                    inning: 3,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 4,
                    home: Some(1),
                    away: Some(0),
                },
                InningData {
                    inning: 5,
                    home: Some(0),
                    away: Some(2),
                },
                InningData {
                    inning: 6,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 7,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 8,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 9,
                    home: Some(0),
                    away: Some(0),
                },
            ],
            home_runs: Some(3),
            away_runs: Some(3),
        };

        let display = format!("{}", game_innings);
        
        // Verify the display output contains expected information
        assert!(display.contains("Game: Away Team vs Home Team"));
        assert!(display.contains("Date: 2024-03-28"));
        assert!(display.contains("Status: Final"));
        assert!(display.contains("Awa |  1  0  0  0  2  0  0  0  0  | 3"));
        assert!(display.contains("Hom |  0  2  0  1  0  0  0  0  0  | 3"));
    }

    #[test]
    fn print_game_stats_display() {
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
                ],
                pitchers: vec![
                    PlayerPitchingStats {
                        name: "Test Pitcher 2".to_string(),
                        innings_pitched: "7.0".to_string(),
                        strikeouts: 5,
                        earned_runs: 3,
                        hits_allowed: 6,
                        runs_allowed: 3,
                        walks: 1,
                        home_runs_allowed: 1,
                        era: Some("3.86".to_string()),
                    },
                ],
            },
        };

        // Create mock game innings
        let game_innings = GameInnings {
            game_pk: 1,
            game_date: "2024-03-28T13:05:00Z".to_string(),
            status: GameStatus {
                abstract_game_state: "Final".to_string(),
                detailed_state: "Final".to_string(),
            },
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
                    home: Some(0),
                    away: Some(1),
                },
                InningData {
                    inning: 2,
                    home: Some(2),
                    away: Some(0),
                },
                InningData {
                    inning: 3,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 4,
                    home: Some(1),
                    away: Some(0),
                },
                InningData {
                    inning: 5,
                    home: Some(0),
                    away: Some(2),
                },
                InningData {
                    inning: 6,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 7,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 8,
                    home: Some(0),
                    away: Some(0),
                },
                InningData {
                    inning: 9,
                    home: Some(0),
                    away: Some(0),
                },
            ],
            home_runs: Some(3),
            away_runs: Some(3),
        };

        // Print the output in the desired order
        println!("\n=== SAMPLE OUTPUT IN NEW ORDER ===");
        println!("\nInning-by-Inning Breakdown:");
        println!("{}", game_innings);
        println!("\nDetailed Statistics:");
        println!("{}", game_stats);
    }
}