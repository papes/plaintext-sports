use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Import our MLB API module
mod mlb;

/// CLI application for plaintext sports
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Optional name to personalize the experience
    #[clap(short, long)]
    name: Option<String>,

    /// MLB player ID to get stats for
    #[clap(short, long)]
    player_id: Option<u32>,

    /// MLB team ID to get stats for
    #[clap(short, long)]
    team_id: Option<u32>,

    /// MLB game ID to get results for
    #[clap(short = 'g', long)]
    game_id: Option<u64>,

    /// Get schedule for the specified team
    #[clap(long)]
    schedule: bool,

    /// Start date for schedule (YYYY-MM-DD)
    #[clap(long)]
    start_date: Option<String>,

    /// End date for schedule (YYYY-MM-DD)
    #[clap(long)]
    end_date: Option<String>,

    /// Get all MLB games being played today
    #[clap(long)]
    todays_games: bool,
    
    /// Get inning-by-inning breakdown for a game
    #[clap(long)]
    innings: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Parse command line arguments
    let args = Args::parse();

    // Greet the user
    if let Some(name) = args.name {
        info!("Hello, {}! Welcome to Plaintext Sports!", name);
    } else {
        info!("Welcome to Plaintext Sports!");
    }

    // Handle player stats request
    if let Some(player_id) = args.player_id {
        info!("Fetching stats for player ID: {}", player_id);
        match mlb::get_player(player_id).await {
            Ok(player_data) => {
                println!("\nPlayer Information:");
                println!("{}", player_data);
            }
            Err(e) => {
                println!("Error fetching player data: {}", e);
            }
        }
    }

    // Handle team stats request
    if let Some(team_id) = args.team_id {
        info!("Fetching stats for team ID: {}", team_id);
        match mlb::get_team(team_id).await {
            Ok(team_data) => {
                println!("\nTeam Information:");
                println!("{}", team_data);
            }
            Err(e) => {
                println!("Error fetching team data: {}", e);
            }
        }

        // Handle schedule request if --schedule flag is provided
        if args.schedule {
            info!("Fetching schedule for team ID: {}", team_id);
            match mlb::get_team_schedule(team_id, args.start_date.clone(), args.end_date.clone()).await {
                Ok(games) => {
                    println!("\nSchedule:");
                    if games.is_empty() {
                        println!("No games scheduled for the specified period.");
                    } else {
                        for (i, game) in games.iter().enumerate() {
                            println!("\nGame {}:", i + 1);
                            println!("{}", game);
                        }
                    }
                }
                Err(e) => {
                    println!("Error fetching schedule data: {}", e);
                }
            }
        }
    }

    // Handle game results request
    if let Some(game_id) = args.game_id {
        info!("Fetching results for game ID: {}", game_id);
        match mlb::get_game(game_id).await {
            Ok(game_data) => {
                println!("\nGame Information:");
                println!("{}", game_data);
            }
            Err(e) => {
                println!("Error fetching game data: {}", e);
            }
        }
        
        // Handle innings request if --innings flag is provided
        if args.innings {
            info!("Fetching inning-by-inning breakdown for game ID: {}", game_id);
            match mlb::get_game_innings(game_id as u32).await {
                Ok(innings_data) => {
                    println!("\nInning-by-Inning Breakdown:");
                    println!("{}", innings_data);
                }
                Err(e) => {
                    println!("Error fetching innings data: {}", e);
                }
            }
        }
    }

    // Handle today's games request
    if args.todays_games {
        info!("Fetching all MLB games scheduled for today");
        match mlb::get_todays_games().await {
            Ok(games) => {
                println!("\nToday's MLB Games:");
                if games.is_empty() {
                    println!("No games scheduled for today.");
                } else {
                    for (i, game) in games.iter().enumerate() {
                        println!("\n==================================================");
                        println!("Game {}: ID {}", i + 1, game.game_pk);
                        println!("==================================================");
                        println!("{}", game);
                        
                        // Only fetch detailed stats for completed games
                        if game.status.abstract_game_state == "Final" {
                            info!("Fetching detailed stats for game ID: {}", game.game_pk);
                            match mlb::get_game_stats(game.game_pk).await {
                                Ok(stats) => {
                                    println!("\nDetailed Statistics:");
                                    println!("{}", stats);
                                }
                                Err(e) => {
                                    println!("Error fetching detailed game stats: {}", e);
                                }
                            }
                            
                            // Fetch inning-by-inning breakdown if --innings flag is provided
                            if args.innings {
                                info!("Fetching inning-by-inning breakdown for game ID: {}", game.game_pk);
                                match mlb::get_game_innings(game.game_pk).await {
                                    Ok(innings_data) => {
                                        println!("\nInning-by-Inning Breakdown:");
                                        println!("{}", innings_data);
                                    }
                                    Err(e) => {
                                        println!("Error fetching innings data: {}", e);
                                    }
                                }
                            }
                        } else {
                            println!("\nDetailed statistics not available for games that haven't been completed.");
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error fetching today's games: {}", e);
            }
        }
    }

    // If no specific request was made, show usage information
    if args.player_id.is_none() && args.team_id.is_none() && args.game_id.is_none() && !args.todays_games {
        println!("\nUsage Examples:");
        println!("  Get player stats: plaintext-sports --player-id 547989");
        println!("  Get team stats: plaintext-sports --team-id 145");
        println!("  Get team schedule: plaintext-sports --team-id 145 --schedule");
        println!("  Get game results: plaintext-sports --game-id 12345");
        println!("  Get inning-by-inning breakdown: plaintext-sports --game-id 12345 --innings");
        println!("  Get all of today's MLB games: plaintext-sports --todays-games");
        println!("  Get all of today's MLB games with inning breakdowns: plaintext-sports --todays-games --innings");
        println!("  Get team schedule for a specific period: plaintext-sports --team-id 145 --schedule --start-date 2025-04-01 --end-date 2025-04-30");
        println!("\nFor more options, use --help");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        // This test ensures that the CLI args are valid
        Args::command().debug_assert();
    }

    #[test]
    fn test_args_parsing() {
        // Test player ID parsing
        let args = Args::parse_from(["plaintext-sports", "--player-id", "547989"]);
        assert_eq!(args.player_id, Some(547989));
        assert_eq!(args.team_id, None);
        assert_eq!(args.game_id, None);
        assert_eq!(args.schedule, false);
        assert_eq!(args.innings, false);

        // Test team ID parsing
        let args = Args::parse_from(["plaintext-sports", "--team-id", "145"]);
        assert_eq!(args.player_id, None);
        assert_eq!(args.team_id, Some(145));
        assert_eq!(args.game_id, None);
        assert_eq!(args.schedule, false);
        assert_eq!(args.innings, false);

        // Test game ID parsing
        let args = Args::parse_from(["plaintext-sports", "--game-id", "123456"]);
        assert_eq!(args.player_id, None);
        assert_eq!(args.team_id, None);
        assert_eq!(args.game_id, Some(123456));
        assert_eq!(args.schedule, false);
        assert_eq!(args.innings, false);

        // Test schedule flag
        let args = Args::parse_from(["plaintext-sports", "--team-id", "145", "--schedule"]);
        assert_eq!(args.team_id, Some(145));
        assert_eq!(args.schedule, true);
        assert_eq!(args.innings, false);

        // Test innings flag
        let args = Args::parse_from(["plaintext-sports", "--game-id", "123456", "--innings"]);
        assert_eq!(args.game_id, Some(123456));
        assert_eq!(args.innings, true);

        // Test date range parsing
        let args = Args::parse_from([
            "plaintext-sports", 
            "--team-id", "145", 
            "--schedule", 
            "--start-date", "2023-04-01", 
            "--end-date", "2023-04-05"
        ]);
        assert_eq!(args.team_id, Some(145));
        assert_eq!(args.schedule, true);
        assert_eq!(args.start_date, Some("2023-04-01".to_string()));
        assert_eq!(args.end_date, Some("2023-04-05".to_string()));

        // Test name parsing
        let args = Args::parse_from(["plaintext-sports", "--name", "John"]);
        assert_eq!(args.name, Some("John".to_string()));
        
        // Test todays_games flag
        let args = Args::parse_from(["plaintext-sports", "--todays-games"]);
        assert_eq!(args.player_id, None);
        assert_eq!(args.team_id, None);
        assert_eq!(args.game_id, None);
        assert_eq!(args.schedule, false);
        assert_eq!(args.todays_games, true);
        assert_eq!(args.innings, false);
        
        // Test todays_games with innings flag
        let args = Args::parse_from(["plaintext-sports", "--todays-games", "--innings"]);
        assert_eq!(args.todays_games, true);
        assert_eq!(args.innings, true);
    }
}
