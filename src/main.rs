use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Import our modules
mod mlb;
mod nba;
mod config;

/// CLI application for plaintext sports
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Optional name to personalize the experience
    #[clap(short, long)]
    name: Option<String>,

    /// Show detailed pitching and hitting statistics
    #[clap(long)]
    detailed_stats: bool,

    /// Get all games being played today for both MLB and NBA
    #[clap(long)]
    todays_games: bool,

    /// Get all games played yesterday for both MLB and NBA
    #[clap(long)]
    yesterday_games: bool,

    /// Filter games by leagues (e.g., --leagues MLB NBA)
    #[clap(long, value_delimiter = ' ')]
    leagues: Option<Vec<String>>,

    /// Start date for schedule (YYYY-MM-DD)
    #[clap(long)]
    start_date: Option<String>,

    /// End date for schedule (YYYY-MM-DD)
    #[clap(long)]
    end_date: Option<String>,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// MLB related commands
    MLB {
        #[clap(subcommand)]
        command: MLBCommand,
    },
    /// NBA related commands
    NBA {
        #[clap(subcommand)]
        command: NBACommand,
    },
}

#[derive(Subcommand, Debug)]
enum MLBCommand {
    /// Get MLB player stats
    Player {
        /// MLB player ID
        #[clap(short, long)]
        id: u32,
    },
    /// Get MLB team stats
    Team {
        /// MLB team ID
        #[clap(short, long)]
        id: u32,
        
        /// Get schedule for the specified team
        #[clap(long)]
        schedule: bool,
    },
    /// Get MLB game results
    Game {
        /// MLB game ID
        #[clap(short, long)]
        id: u64,
    },
    /// Get all MLB games being played today
    TodaysGames,
    /// Get all MLB games played yesterday
    YesterdayGames,
}

#[derive(Subcommand, Debug)]
enum NBACommand {
    /// Get NBA player stats
    Player {
        /// NBA player ID
        #[clap(short, long)]
        id: u32,
    },
    /// Get NBA team stats
    Team {
        /// NBA team ID
        #[clap(short, long)]
        id: u32,
        
        /// Get schedule for the specified team
        #[clap(long)]
        schedule: bool,
    },
    /// Get all NBA games being played today
    TodaysGames,
    /// Get all NBA games played yesterday
    YesterdayGames,
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
    if let Some(name) = args.name.as_ref() {
        info!("Hello, {}! Welcome to Plaintext Sports!", name);
    } else {
        info!("Welcome to Plaintext Sports!");
    }

    // Determine which leagues to fetch based on the leagues argument
    let fetch_mlb = args.leagues.as_ref().map_or(true, |leagues| leagues.iter().any(|l| l.to_uppercase() == "MLB"));
    let fetch_nba = args.leagues.as_ref().map_or(true, |leagues| leagues.iter().any(|l| l.to_uppercase() == "NBA"));

    // Handle combined commands
    if args.todays_games {
        info!("Fetching today's games for selected leagues");
        
        // Fetch MLB games if selected
        if fetch_mlb {
            match mlb::get_todays_games().await {
                Ok(games) => {
                    println!("\nToday's MLB Games:");
                    if games.is_empty() {
                        println!("No MLB games scheduled for today.");
                    } else {
                        for (i, game) in games.iter().enumerate() {
                            println!("\n==================================================");
                            println!("Game {}: ID {}", i + 1, game.game_pk);
                            println!("==================================================");
                            println!("{}", game);
                            
                            // Only fetch data for completed games
                            if game.status.abstract_game_state == "Final" {
                                // Always fetch inning-by-inning breakdown by default
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
                                
                                // Only fetch detailed stats if the flag is provided
                                if args.detailed_stats {
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
                                }
                            } else {
                                println!("\nDetailed information not available for games that haven't been completed.");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error fetching today's MLB games: {}", e);
                }
            }
        }
        
        // Fetch NBA games if selected
        if fetch_nba {
            match nba::get_todays_games().await {
                Ok(games) => {
                    println!("\nToday's NBA Games:");
                    if games.is_empty() {
                        println!("No NBA games scheduled for today.");
                    } else {
                        for (i, game) in games.iter().enumerate() {
                            println!("\n==================================================");
                            println!("Game {}: ID {}", i + 1, game.id);
                            println!("==================================================");
                            println!("{}", game);
                            
                            // Fetch player stats for completed games
                            if game.status == "Final" {
                                info!("Fetching player stats for NBA game ID: {}", game.id);
                                match nba::get_game_player_stats(game.id).await {
                                    Ok(stats) => {
                                        println!("\nPlayer Statistics:");
                                        println!("{}", nba::display_game_player_stats(game.id, &stats));
                                    }
                                    Err(e) => {
                                        println!("Error fetching player stats: {}", e);
                                    }
                                }
                            } else {
                                println!("\nDetailed player statistics not available for games that haven't been completed.");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error fetching today's NBA games: {}", e);
                }
            }
        }
    }

    if args.yesterday_games {
        info!("Fetching yesterday's games for selected leagues");
        
        // Fetch MLB games from yesterday if selected
        if fetch_mlb {
            match mlb::get_yesterdays_games().await {
                Ok(games) => {
                    println!("\nYesterday's MLB Games:");
                    if games.is_empty() {
                        println!("No MLB games played yesterday.");
                    } else {
                        for (i, game) in games.iter().enumerate() {
                            println!("\n==================================================");
                            println!("Game {}: ID {}", i + 1, game.game_pk);
                            println!("==================================================");
                            println!("{}", game);
                            
                            // Only fetch data for completed games
                            if game.status.abstract_game_state == "Final" {
                                // Always fetch inning-by-inning breakdown by default
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
                                
                                // Only fetch detailed stats if the flag is provided
                                if args.detailed_stats {
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
                                }
                            } else {
                                println!("\nDetailed information not available for games that haven't been completed.");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error fetching yesterday's MLB games: {}", e);
                }
            }
        }
        
        // Fetch NBA games from yesterday if selected
        if fetch_nba {
            match nba::get_yesterdays_games().await {
                Ok(games) => {
                    println!("\nYesterday's NBA Games:");
                    if games.is_empty() {
                        println!("No NBA games played yesterday.");
                    } else {
                        for (i, game) in games.iter().enumerate() {
                            println!("\n==================================================");
                            println!("Game {}: ID {}", i + 1, game.id);
                            println!("==================================================");
                            println!("{}", game);
                            
                            // Fetch player stats for completed games
                            if game.status == "Final" {
                                info!("Fetching player stats for NBA game ID: {}", game.id);
                                match nba::get_game_player_stats(game.id).await {
                                    Ok(stats) => {
                                        println!("\nPlayer Statistics:");
                                        println!("{}", nba::display_game_player_stats(game.id, &stats));
                                    }
                                    Err(e) => {
                                        println!("Error fetching player stats: {}", e);
                                    }
                                }
                            } else {
                                println!("\nDetailed player statistics not available for games that haven't been completed.");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error fetching yesterday's NBA games: {}", e);
                }
            }
        }
    }

    // Handle subcommands
    if let Some(ref command) = args.command {
        match command {
            Command::MLB { command } => {
                match command {
                    MLBCommand::Player { id } => {
                        info!("Fetching stats for MLB player ID: {}", id);
                        match mlb::get_player(*id).await {
                            Ok(player_data) => {
                                println!("\nMLB Player Information:");
                                println!("{}", player_data);
                            }
                            Err(e) => {
                                println!("Error fetching MLB player data: {}", e);
                            }
                        }
                    },
                    MLBCommand::Team { id, schedule } => {
                        info!("Fetching stats for MLB team ID: {}", id);
                        match mlb::get_team(*id).await {
                            Ok(team_data) => {
                                println!("\nMLB Team Information:");
                                println!("{}", team_data);
                            }
                            Err(e) => {
                                println!("Error fetching MLB team data: {}", e);
                            }
                        }

                        // Handle schedule request if --schedule flag is provided
                        if *schedule {
                            info!("Fetching schedule for MLB team ID: {}", id);
                            match mlb::get_team_schedule(*id, args.start_date.clone(), args.end_date.clone()).await {
                                Ok(games) => {
                                    println!("\nMLB Schedule:");
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
                                    println!("Error fetching MLB schedule: {}", e);
                                }
                            }
                        }
                    },
                    MLBCommand::Game { id } => {
                        info!("Fetching results for MLB game ID: {}", id);
                        match mlb::get_game(*id).await {
                            Ok(game_data) => {
                                println!("\nGame Information:");
                                println!("{}", game_data);
                            }
                            Err(e) => {
                                println!("Error fetching game data: {}", e);
                            }
                        }
                        
                        // Always fetch inning-by-inning breakdown by default
                        info!("Fetching inning-by-inning breakdown for game ID: {}", id);
                        match mlb::get_game_innings(*id as u32).await {
                            Ok(innings_data) => {
                                println!("\nInning-by-Inning Breakdown:");
                                println!("{}", innings_data);
                            }
                            Err(e) => {
                                println!("Error fetching innings data: {}", e);
                            }
                        }
                        
                        // Only fetch detailed stats if the flag is provided
                        if args.detailed_stats {
                            info!("Fetching detailed statistics for game ID: {}", id);
                            match mlb::get_game_stats(*id as u32).await {
                                Ok(stats) => {
                                    println!("\nDetailed Statistics:");
                                    println!("{}", stats);
                                }
                                Err(e) => {
                                    println!("Error fetching detailed game stats: {}", e);
                                }
                            }
                        }
                    },
                    MLBCommand::TodaysGames => {
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
                                        
                                        // Only fetch data for completed games
                                        if game.status.abstract_game_state == "Final" {
                                            // Always fetch inning-by-inning breakdown by default
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
                                            
                                            // Only fetch detailed stats if the flag is provided
                                            if args.detailed_stats {
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
                                            }
                                        } else {
                                            println!("\nDetailed information not available for games that haven't been completed.");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error fetching today's games: {}", e);
                            }
                        }
                    },
                    MLBCommand::YesterdayGames => {
                        info!("Fetching all MLB games from yesterday");
                        match mlb::get_yesterdays_games().await {
                            Ok(games) => {
                                println!("\nYesterday's MLB Games:");
                                if games.is_empty() {
                                    println!("No games played yesterday.");
                                } else {
                                    for (i, game) in games.iter().enumerate() {
                                        println!("\n==================================================");
                                        println!("Game {}: ID {}", i + 1, game.game_pk);
                                        println!("==================================================");
                                        println!("{}", game);
                                        
                                        // Only fetch data for completed games
                                        if game.status.abstract_game_state == "Final" {
                                            // Always fetch inning-by-inning breakdown by default
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
                                            
                                            // Only fetch detailed stats if the flag is provided
                                            if args.detailed_stats {
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
                                            }
                                        } else {
                                            println!("\nDetailed information not available for games that haven't been completed.");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error fetching yesterday's games: {}", e);
                            }
                        }
                    },
                }
            },
            Command::NBA { command } => {
                match command {
                    NBACommand::Player { id } => {
                        info!("Fetching stats for NBA player ID: {}", id);
                        match nba::get_player(*id).await {
                            Ok(player_data) => {
                                println!("\nNBA Player Information:");
                                println!("{}", player_data);
                            }
                            Err(e) => {
                                println!("Error fetching NBA player data: {}", e);
                            }
                        }
                    },
                    NBACommand::Team { id, schedule } => {
                        info!("Fetching stats for NBA team ID: {}", id);
                        match nba::get_team(*id).await {
                            Ok(team_data) => {
                                println!("\nNBA Team Information:");
                                println!("{}", team_data);
                            }
                            Err(e) => {
                                println!("Error fetching NBA team data: {}", e);
                            }
                        }

                        // Handle schedule request if --schedule flag is provided
                        if *schedule {
                            info!("Fetching schedule for NBA team ID: {}", id);
                            match nba::get_team_games(*id, args.start_date.clone(), args.end_date.clone()).await {
                                Ok(games) => {
                                    println!("\nNBA Schedule:");
                                    if games.is_empty() {
                                        println!("No games scheduled for the specified period.");
                                    } else {
                                        for (i, game) in games.iter().enumerate() {
                                            println!("\n==================================================");
                                            println!("Game {}: ID {}", i + 1, game.id);
                                            println!("==================================================");
                                            println!("{}", game);
                                            
                                            // Fetch player stats for completed games
                                            if game.status == "Final" {
                                                info!("Fetching player stats for NBA game ID: {}", game.id);
                                                match nba::get_game_player_stats(game.id).await {
                                                    Ok(stats) => {
                                                        println!("\nPlayer Statistics:");
                                                        println!("{}", nba::display_game_player_stats(game.id, &stats));
                                                    }
                                                    Err(e) => {
                                                        println!("Error fetching player stats: {}", e);
                                                    }
                                                }
                                            } else {
                                                println!("\nDetailed player statistics not available for games that haven't been completed.");
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Error fetching NBA schedule: {}", e);
                                }
                            }
                        }
                    },
                    NBACommand::TodaysGames => {
                        info!("Fetching all NBA games for today");
                        match nba::get_todays_games().await {
                            Ok(games) => {
                                println!("\nToday's NBA Games:");
                                if games.is_empty() {
                                    println!("No NBA games scheduled for today.");
                                } else {
                                    for (i, game) in games.iter().enumerate() {
                                        println!("\n==================================================");
                                        println!("Game {}: ID {}", i + 1, game.id);
                                        println!("==================================================");
                                        println!("{}", game);
                                        
                                        // Fetch player stats for completed games
                                        if game.status == "Final" {
                                            info!("Fetching player stats for NBA game ID: {}", game.id);
                                            match nba::get_game_player_stats(game.id).await {
                                                Ok(stats) => {
                                                    println!("\nPlayer Statistics:");
                                                    println!("{}", nba::display_game_player_stats(game.id, &stats));
                                                }
                                                Err(e) => {
                                                    println!("Error fetching player stats: {}", e);
                                                }
                                            }
                                        } else {
                                            println!("\nDetailed player statistics not available for games that haven't been completed.");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error fetching today's NBA games: {}", e);
                            }
                        }
                    },
                    NBACommand::YesterdayGames => {
                        info!("Fetching all NBA games from yesterday");
                        match nba::get_yesterdays_games().await {
                            Ok(games) => {
                                println!("\nYesterday's NBA Games:");
                                if games.is_empty() {
                                    println!("No NBA games played yesterday.");
                                } else {
                                    for (i, game) in games.iter().enumerate() {
                                        println!("\n==================================================");
                                        println!("Game {}: ID {}", i + 1, game.id);
                                        println!("==================================================");
                                        println!("{}", game);
                                        
                                        // Fetch player stats for completed games
                                        if game.status == "Final" {
                                            info!("Fetching player stats for NBA game ID: {}", game.id);
                                            match nba::get_game_player_stats(game.id).await {
                                                Ok(stats) => {
                                                    println!("\nPlayer Statistics:");
                                                    println!("{}", nba::display_game_player_stats(game.id, &stats));
                                                }
                                                Err(e) => {
                                                    println!("Error fetching player stats: {}", e);
                                                }
                                            }
                                        } else {
                                            println!("\nDetailed player statistics not available for games that haven't been completed.");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error fetching yesterday's NBA games: {}", e);
                            }
                        }
                    },
                }
            },
        }
    }

    // If no specific request was made, show usage information
    if args.command.is_none() && !args.todays_games && !args.yesterday_games {
        println!("\nUsage Examples:");
        println!("  Get all of today's games (MLB and NBA): plaintext-sports --todays-games");
        println!("  Get only MLB games for today: plaintext-sports --todays-games --leagues MLB");
        println!("  Get only NBA games for today: plaintext-sports --todays-games --leagues NBA");
        println!("  Get all of yesterday's games (MLB and NBA): plaintext-sports --yesterday-games");
        println!("  Get only MLB games from yesterday: plaintext-sports --yesterday-games --leagues MLB");
        println!("  Get only NBA games from yesterday: plaintext-sports --yesterday-games --leagues NBA");
        println!("  Get all of today's games with detailed stats: plaintext-sports --todays-games --detailed-stats");
        println!("\nMLB Commands:");
        println!("  Get player stats: plaintext-sports mlb player --id 547989");
        println!("  Get team stats: plaintext-sports mlb team --id 145");
        println!("  Get team schedule: plaintext-sports mlb team --id 145 --schedule");
        println!("  Get game results: plaintext-sports mlb game --id 12345");
        println!("  Get game results with detailed stats: plaintext-sports mlb game --id 12345 --detailed-stats");
        println!("  Get all of today's MLB games: plaintext-sports mlb todays-games");
        println!("  Get all of yesterday's MLB games: plaintext-sports mlb yesterday-games");
        println!("\nNBA Commands:");
        println!("  Get player stats: plaintext-sports nba player --id 237");
        println!("  Get team stats: plaintext-sports nba team --id 14");
        println!("  Get team schedule: plaintext-sports nba team --id 14 --schedule");
        println!("  Get all of today's NBA games: plaintext-sports nba todays-games");
        println!("  Get all of yesterday's NBA games: plaintext-sports nba yesterday-games");
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
        // Test name parsing
        let args = Args::parse_from(["plaintext-sports", "--name", "John"]);
        assert_eq!(args.name, Some("John".to_string()));
        
        // Test detailed_stats flag
        let args = Args::parse_from(["plaintext-sports", "--detailed-stats"]);
        assert_eq!(args.detailed_stats, true);
        
        // Test todays_games flag
        let args = Args::parse_from(["plaintext-sports", "--todays-games"]);
        assert_eq!(args.todays_games, true);
        
        // Test yesterday_games flag
        let args = Args::parse_from(["plaintext-sports", "--yesterday-games"]);
        assert_eq!(args.yesterday_games, true);
        
        // Test MLB player command
        let args = Args::parse_from(["plaintext-sports", "mlb", "player", "--id", "547989"]);
        match args.command {
            Some(Command::MLB { command }) => {
                match command {
                    MLBCommand::Player { id } => assert_eq!(id, 547989),
                    _ => panic!("Expected MLBCommand::Player"),
                }
            },
            _ => panic!("Expected Command::MLB"),
        }
        
        // Test MLB team command
        let args = Args::parse_from(["plaintext-sports", "mlb", "team", "--id", "145"]);
        match args.command {
            Some(Command::MLB { command }) => {
                match command {
                    MLBCommand::Team { id, schedule } => {
                        assert_eq!(id, 145);
                        assert_eq!(schedule, false);
                    },
                    _ => panic!("Expected MLBCommand::Team"),
                }
            },
            _ => panic!("Expected Command::MLB"),
        }
        
        // Test MLB team schedule command
        let args = Args::parse_from(["plaintext-sports", "mlb", "team", "--id", "145", "--schedule"]);
        match args.command {
            Some(Command::MLB { command }) => {
                match command {
                    MLBCommand::Team { id, schedule } => {
                        assert_eq!(id, 145);
                        assert_eq!(schedule, true);
                    },
                    _ => panic!("Expected MLBCommand::Team"),
                }
            },
            _ => panic!("Expected Command::MLB"),
        }
        
        // Test MLB todays games command
        let args = Args::parse_from(["plaintext-sports", "mlb", "todays-games"]);
        match args.command {
            Some(Command::MLB { command }) => {
                match command {
                    MLBCommand::TodaysGames => {},
                    _ => panic!("Expected MLBCommand::TodaysGames"),
                }
            },
            _ => panic!("Expected Command::MLB"),
        }
        
        // Test MLB yesterday games command
        let args = Args::parse_from(["plaintext-sports", "mlb", "yesterday-games"]);
        match args.command {
            Some(Command::MLB { command }) => {
                match command {
                    MLBCommand::YesterdayGames => {},
                    _ => panic!("Expected MLBCommand::YesterdayGames"),
                }
            },
            _ => panic!("Expected Command::MLB"),
        }
        
        // Test NBA player command
        let args = Args::parse_from(["plaintext-sports", "nba", "player", "--id", "237"]);
        match args.command {
            Some(Command::NBA { command }) => {
                match command {
                    NBACommand::Player { id } => assert_eq!(id, 237),
                    _ => panic!("Expected NBACommand::Player"),
                }
            },
            _ => panic!("Expected Command::NBA"),
        }
        
        // Test NBA team command
        let args = Args::parse_from(["plaintext-sports", "nba", "team", "--id", "14"]);
        match args.command {
            Some(Command::NBA { command }) => {
                match command {
                    NBACommand::Team { id, schedule } => {
                        assert_eq!(id, 14);
                        assert_eq!(schedule, false);
                    },
                    _ => panic!("Expected NBACommand::Team"),
                }
            },
            _ => panic!("Expected Command::NBA"),
        }
        
        // Test NBA team schedule command
        let args = Args::parse_from(["plaintext-sports", "nba", "team", "--id", "14", "--schedule"]);
        match args.command {
            Some(Command::NBA { command }) => {
                match command {
                    NBACommand::Team { id, schedule } => {
                        assert_eq!(id, 14);
                        assert_eq!(schedule, true);
                    },
                    _ => panic!("Expected NBACommand::Team"),
                }
            },
            _ => panic!("Expected Command::NBA"),
        }
        
        // Test NBA todays games command
        let args = Args::parse_from(["plaintext-sports", "nba", "todays-games"]);
        match args.command {
            Some(Command::NBA { command }) => {
                match command {
                    NBACommand::TodaysGames => {},
                    _ => panic!("Expected NBACommand::TodaysGames"),
                }
            },
            _ => panic!("Expected Command::NBA"),
        }
        
        // Test NBA yesterday games command
        let args = Args::parse_from(["plaintext-sports", "nba", "yesterday-games"]);
        match args.command {
            Some(Command::NBA { command }) => {
                match command {
                    NBACommand::YesterdayGames => {},
                    _ => panic!("Expected NBACommand::YesterdayGames"),
                }
            },
            _ => panic!("Expected Command::NBA"),
        }
        
        // Test combined flags
        let args = Args::parse_from(["plaintext-sports", "--todays-games", "--detailed-stats"]);
        assert_eq!(args.todays_games, true);
        assert_eq!(args.detailed_stats, true);
        
        let args = Args::parse_from(["plaintext-sports", "--yesterday-games", "--detailed-stats"]);
        assert_eq!(args.yesterday_games, true);
        assert_eq!(args.detailed_stats, true);
    }
}
