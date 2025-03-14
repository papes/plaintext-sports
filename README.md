# Plaintext Sports

A command-line application for plaintext sports.

## Features

- Modern CLI interface using clap
- Async runtime with tokio
- Proper error handling with anyhow
- Structured logging with tracing
- Centralized configuration system
- MLB stats and scores via the MLB Stats API
- NBA stats and scores via the balldontlie API
- Team schedules with game details
- Today's MLB and NBA games with detailed statistics
- Yesterday's MLB and NBA games with detailed statistics
- League filtering for combined commands (MLB, NBA, or both)
- Comprehensive game statistics including batting and pitching details for MLB
- Player statistics for NBA games, ordered by away team followed by home team
- Clear separation between MLB and NBA commands

## Installation

Make sure you have Rust installed on your system. Then:

```bash
git clone <repository-url>
cd plaintext-sports
cargo build --release
```

The binary will be available in `target/release/plaintext-sports`

## API Keys

This application requires an API key for the balldontlie NBA API. You can obtain a free API key by signing up at [balldontlie.io](https://balldontlie.io/).

Once you have your API key, set it as an environment variable:

```bash
export NBA_API_KEY="your-api-key-here"
```

## Usage

```bash
# Run the application with a name
plaintext-sports --name YourName

# Combined Commands
# ----------------

# Get all of today's games (MLB and NBA)
plaintext-sports --todays-games

# Get only MLB games for today
plaintext-sports --todays-games --leagues MLB

# Get only NBA games for today
plaintext-sports --todays-games --leagues NBA

# Get all of yesterday's games (MLB and NBA)
plaintext-sports --yesterday-games

# Get only MLB games from yesterday
plaintext-sports --yesterday-games --leagues MLB

# Get only NBA games from yesterday
plaintext-sports --yesterday-games --leagues NBA

# Get all of today's games with detailed stats
plaintext-sports --todays-games --detailed-stats

# Get only MLB games for today with detailed stats
plaintext-sports --todays-games --leagues MLB --detailed-stats

# MLB Commands
# -----------

# Get MLB player stats (Jose Abreu's ID: 547989)
plaintext-sports mlb player --id 547989

# Get MLB team stats (Chicago White Sox ID: 145)
plaintext-sports mlb team --id 145

# Get MLB team schedule for the current month
plaintext-sports mlb team --id 145 --schedule

# Get MLB team schedule for a specific period
plaintext-sports mlb team --id 145 --schedule --start-date 2025-04-01 --end-date 2025-04-30

# Get all of today's MLB games with detailed statistics
plaintext-sports mlb todays-games --detailed-stats

# Get all of yesterday's MLB games with detailed statistics
plaintext-sports mlb yesterday-games --detailed-stats

# Get specific MLB game results
plaintext-sports mlb game --id 12345

# NBA Commands
# -----------

# Get NBA player stats (LeBron James's ID: 237)
plaintext-sports nba player --id 237

# Get NBA team stats (Los Angeles Lakers ID: 14)
plaintext-sports nba team --id 14

# Get NBA team schedule
plaintext-sports nba team --id 14 --schedule

# Get NBA team schedule for a specific period
plaintext-sports nba team --id 14 --schedule --start-date 2025-04-01 --end-date 2025-04-30

# Get all of today's NBA games
plaintext-sports nba todays-games

# Get all of yesterday's NBA games
plaintext-sports nba yesterday-games

# Get help
plaintext-sports --help
```

## Configuration

The application uses a centralized configuration system that can be customized through environment variables:

```bash
# Override the MLB API base URL
export MLB_API_BASE_URL="https://alternate-statsapi.mlb.com/api/v1"
plaintext-sports mlb todays-games

# Override the NBA API base URL
export NBA_API_BASE_URL="https://alternate-balldontlie.io/api/v1"
plaintext-sports nba todays-games

# Set your NBA API key
export NBA_API_KEY="your-api-key-here"
plaintext-sports nba todays-games
```

Available configuration options:

| Environment Variable | Default Value | Description |
|---------------------|---------------|-------------|
| MLB_API_BASE_URL | https://statsapi.mlb.com/api/v1 | Base URL for the MLB Stats API |
| NBA_API_BASE_URL | https://api.balldontlie.io/v1 | Base URL for the balldontlie NBA API |
| NBA_API_KEY | (none) | API key for the balldontlie NBA API (required for NBA features) |

## Game Statistics

When using the `--todays-games` or `--yesterday-games` flags, the application provides:

- Basic game information (date, status, teams, scores, venue)
- Detailed team statistics for MLB games:
  - Batting stats (runs, hits, home runs, RBIs, stolen bases, batting averages)
  - Pitching stats (innings pitched, hits allowed, runs allowed, strikeouts, ERA)
  - Top batters with their performance (hits, at-bats, home runs, RBIs)
  - Top pitchers with their performance (innings pitched, strikeouts, earned runs)
- Player statistics for NBA games:
  - Away team players followed by home team players
  - Players sorted by points scored (descending)
  - Individual stats including points, rebounds, assists, steals, blocks, and shooting percentages

Detailed statistics are only available for completed games.

## MLB Team IDs

Some common MLB team IDs:
- 145: Chicago White Sox
- 112: Chicago Cubs
- 133: Oakland Athletics
- 137: San Francisco Giants
- 147: New York Yankees
- 146: Miami Marlins
- 121: New York Mets

## NBA Team IDs

Some common NBA team IDs:
- 1: Atlanta Hawks
- 2: Boston Celtics
- 3: Brooklyn Nets
- 4: Charlotte Hornets
- 5: Chicago Bulls
- 6: Cleveland Cavaliers
- 7: Dallas Mavericks
- 8: Denver Nuggets
- 9: Detroit Pistons
- 10: Golden State Warriors
- 11: Houston Rockets
- 12: Indiana Pacers
- 13: LA Clippers
- 14: Los Angeles Lakers
- 15: Memphis Grizzlies
- 16: Miami Heat
- 17: Milwaukee Bucks
- 18: Minnesota Timberwolves
- 19: New Orleans Pelicans
- 20: New York Knicks
- 21: Oklahoma City Thunder
- 22: Orlando Magic
- 23: Philadelphia 76ers
- 24: Phoenix Suns
- 25: Portland Trail Blazers
- 26: Sacramento Kings
- 27: San Antonio Spurs
- 28: Toronto Raptors
- 29: Utah Jazz
- 30: Washington Wizards

## Development

To run the application in development mode:

```bash
cargo run -- --name YourName
cargo run -- --todays-games
cargo run -- --todays-games --leagues MLB
cargo run -- --todays-games --leagues NBA
cargo run -- --yesterday-games
cargo run -- --yesterday-games --leagues MLB
cargo run -- --yesterday-games --leagues NBA
cargo run -- mlb player --id 547989
cargo run -- mlb team --id 145 --schedule
cargo run -- mlb todays-games
cargo run -- nba player --id 237
cargo run -- nba team --id 14
cargo run -- nba todays-games

# With custom configuration
MLB_API_BASE_URL="https://alternate-statsapi.mlb.com/api/v1" cargo run -- mlb todays-games
NBA_API_BASE_URL="https://api.balldontlie.io/v1" NBA_API_KEY="your-api-key-here" cargo run -- nba todays-games
```

## Data Sources

This application uses the MLB Stats API and the balldontlie NBA API. This project and its authors are not affiliated with MLB, NBA, or any MLB/NBA team. Use of MLB data is subject to the notice posted at http://gdx.mlb.com/components/copyright.txt.

## License

This project is licensed under the MIT License. 