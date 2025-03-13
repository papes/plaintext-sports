# Plaintext Sports

A command-line application for plaintext sports.

## Features

- Modern CLI interface using clap
- Async runtime with tokio
- Proper error handling with anyhow
- Structured logging with tracing
- MLB stats and scores via the MLB Stats API
- Team schedules with game details
- Today's MLB games with detailed statistics
- Comprehensive game statistics including batting and pitching details

## Installation

Make sure you have Rust installed on your system. Then:

```bash
git clone <repository-url>
cd plaintext-sports
cargo build --release
```

The binary will be available in `target/release/plaintext-sports`

## Usage

```bash
# Run the application with a name
plaintext-sports --name YourName

# Get MLB player stats (Jose Abreu's ID: 547989)
plaintext-sports --player-id 547989

# Get MLB team stats (Chicago White Sox ID: 145)
plaintext-sports --team-id 145

# Get team schedule for the current month
plaintext-sports --team-id 145 --schedule

# Get team schedule for a specific period
plaintext-sports --team-id 145 --schedule --start-date 2025-04-01 --end-date 2025-04-30

# Get all of today's MLB games with detailed statistics
plaintext-sports --todays-games

# Get specific game results
plaintext-sports --game-id 12345

# Get help
plaintext-sports --help
```

## Game Statistics

When using the `--todays-games` flag, the application provides:

- Basic game information (date, status, teams, scores, venue)
- Detailed team statistics:
  - Batting stats (runs, hits, home runs, RBIs, stolen bases, batting averages)
  - Pitching stats (innings pitched, hits allowed, runs allowed, strikeouts, ERA)
  - Top batters with their performance (hits, at-bats, home runs, RBIs)
  - Top pitchers with their performance (innings pitched, strikeouts, earned runs)

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

## Development

To run the application in development mode:

```bash
cargo run -- --name YourName
cargo run -- --player-id 547989
cargo run -- --team-id 145 --schedule
cargo run -- --todays-games
```

## Data Source

This application uses the MLB Stats API. This project and its authors are not affiliated with MLB or any MLB team. Use of MLB data is subject to the notice posted at http://gdx.mlb.com/components/copyright.txt.

## License

This project is licensed under the MIT License. 