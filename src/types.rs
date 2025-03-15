use std::fmt;
use std::str::FromStr;
use crate::error::{Result, SportError};

/// Represents a team identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TeamId(pub u32);

impl FromStr for TeamId {
    type Err = SportError;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<u32>()
            .map(TeamId)
            .map_err(|_| SportError::TeamNotFound(format!("Invalid team ID: {}", s)))
    }
}

/// Represents a game identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameId(pub u32);

impl FromStr for GameId {
    type Err = SportError;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<u32>()
            .map(GameId)
            .map_err(|_| SportError::FetchError(format!("Invalid game ID: {}", s)))
    }
}

/// Represents a game score
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Score(pub u32);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a batting average or similar statistic
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Average(pub f32);

impl Average {
    /// Creates a new Average from a float, ensuring it's between 0 and 1
    pub fn new(value: f32) -> Result<Self> {
        if value < 0.0 || value > 1.0 {
            Err(SportError::FetchError(format!(
                "Invalid average value: {}. Must be between 0 and 1",
                value
            )))
        } else {
            Ok(Average(value))
        }
    }

    /// Formats the average as a three-digit string (e.g., ".333")
    pub fn format(&self) -> String {
        format!(".{:03}", (self.0 * 1000.0).round() as u32)
    }
}

impl fmt::Display for Average {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Represents innings pitched
#[derive(Debug, Clone)]
pub struct InningsPitched {
    pub complete: u32,
    pub partial: u32,
}

impl InningsPitched {
    /// Creates a new InningsPitched from a string (e.g., "6.2")
    pub fn new(innings: &str) -> Result<Self> {
        let parts: Vec<&str> = innings.split('.').collect();
        match parts.len() {
            1 => Ok(InningsPitched {
                complete: parts[0].parse().map_err(|_| {
                    SportError::FetchError(format!("Invalid innings format: {}", innings))
                })?,
                partial: 0,
            }),
            2 => {
                let complete = parts[0].parse().map_err(|_| {
                    SportError::FetchError(format!("Invalid innings format: {}", innings))
                })?;
                let partial = parts[1].parse().map_err(|_| {
                    SportError::FetchError(format!("Invalid innings format: {}", innings))
                })?;
                if partial >= 3 {
                    return Err(SportError::FetchError(format!(
                        "Invalid partial innings: {}. Must be less than 3",
                        partial
                    )));
                }
                Ok(InningsPitched { complete, partial })
            }
            _ => Err(SportError::FetchError(format!(
                "Invalid innings format: {}",
                innings
            ))),
        }
    }

    /// Converts innings pitched to a float value
    pub fn as_float(&self) -> f32 {
        self.complete as f32 + (self.partial as f32 / 3.0)
    }
}

impl fmt::Display for InningsPitched {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.partial == 0 {
            write!(f, "{}.0", self.complete)
        } else {
            write!(f, "{}.{}", self.complete, self.partial)
        }
    }
}

/// Represents a player's position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Pitcher,
    Catcher,
    FirstBase,
    SecondBase,
    ThirdBase,
    Shortstop,
    LeftField,
    CenterField,
    RightField,
    DesignatedHitter,
    Unknown,
}

impl Position {
    /// Returns the abbreviation for the position
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Position::Pitcher => "P",
            Position::Catcher => "C",
            Position::FirstBase => "1B",
            Position::SecondBase => "2B",
            Position::ThirdBase => "3B",
            Position::Shortstop => "SS",
            Position::LeftField => "LF",
            Position::CenterField => "CF",
            Position::RightField => "RF",
            Position::DesignatedHitter => "DH",
            Position::Unknown => "??",
        }
    }
}

impl FromStr for Position {
    type Err = SportError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "P" => Ok(Position::Pitcher),
            "C" => Ok(Position::Catcher),
            "1B" => Ok(Position::FirstBase),
            "2B" => Ok(Position::SecondBase),
            "3B" => Ok(Position::ThirdBase),
            "SS" => Ok(Position::Shortstop),
            "LF" => Ok(Position::LeftField),
            "CF" => Ok(Position::CenterField),
            "RF" => Ok(Position::RightField),
            "DH" => Ok(Position::DesignatedHitter),
            _ => Ok(Position::Unknown),
        }
    }
}

/// Represents a player's batting side
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattingSide {
    Left,
    Right,
    Switch,
    Unknown,
}

impl FromStr for BattingSide {
    type Err = SportError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "L" | "LEFT" => Ok(BattingSide::Left),
            "R" | "RIGHT" => Ok(BattingSide::Right),
            "S" | "SWITCH" => Ok(BattingSide::Switch),
            _ => Ok(BattingSide::Unknown),
        }
    }
}

/// Represents a player's throwing hand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThrowingSide {
    Left,
    Right,
    Unknown,
}

impl FromStr for ThrowingSide {
    type Err = SportError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "L" | "LEFT" => Ok(ThrowingSide::Left),
            "R" | "RIGHT" => Ok(ThrowingSide::Right),
            _ => Ok(ThrowingSide::Unknown),
        }
    }
}

/// Represents a win-loss record
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record {
    pub wins: u32,
    pub losses: u32,
}

impl Record {
    /// Creates a new Record
    pub fn new(wins: u32, losses: u32) -> Self {
        Self { wins, losses }
    }

    /// Calculates winning percentage
    pub fn winning_percentage(&self) -> f32 {
        if self.wins + self.losses == 0 {
            0.0
        } else {
            self.wins as f32 / (self.wins + self.losses) as f32
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.wins, self.losses)
    }
}

impl FromStr for Record {
    type Err = SportError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(SportError::FetchError(format!(
                "Invalid record format: {}",
                s
            )));
        }

        let wins = parts[0].parse().map_err(|_| {
            SportError::FetchError(format!("Invalid wins in record: {}", s))
        })?;
        let losses = parts[1].parse().map_err(|_| {
            SportError::FetchError(format!("Invalid losses in record: {}", s))
        })?;

        Ok(Record::new(wins, losses))
    }
}

/// Represents a player's uniform number
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniformNumber(pub String);

impl UniformNumber {
    /// Creates a new UniformNumber, validating the format
    pub fn new(number: impl Into<String>) -> Result<Self> {
        let number = number.into();
        if number.chars().all(|c| c.is_ascii_digit()) {
            Ok(UniformNumber(number))
        } else {
            Err(SportError::FetchError(format!(
                "Invalid uniform number: {}",
                number
            )))
        }
    }
}

impl fmt::Display for UniformNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_id_from_str() {
        assert!(TeamId::from_str("123").is_ok());
        assert!(TeamId::from_str("invalid").is_err());
    }

    #[test]
    fn test_game_id_from_str() {
        assert!(GameId::from_str("456").is_ok());
        assert!(GameId::from_str("invalid").is_err());
    }

    #[test]
    fn test_score_display() {
        assert_eq!(Score(5).to_string(), "5");
    }

    #[test]
    fn test_average() {
        assert!(Average::new(0.333).is_ok());
        assert!(Average::new(1.5).is_err());
        assert_eq!(Average::new(0.333).unwrap().format(), ".333");
    }

    #[test]
    fn test_innings_pitched() {
        let ip = InningsPitched::new("6.2").unwrap();
        assert_eq!(ip.complete, 6);
        assert_eq!(ip.partial, 2);
        assert_eq!(ip.to_string(), "6.2");
        assert!((ip.as_float() - 6.666667).abs() < 0.0001);

        assert!(InningsPitched::new("6.3").is_err());
        assert!(InningsPitched::new("invalid").is_err());
    }

    #[test]
    fn test_position_from_str() {
        assert_eq!(Position::from_str("P").unwrap(), Position::Pitcher);
        assert_eq!(Position::from_str("SS").unwrap(), Position::Shortstop);
        assert_eq!(Position::from_str("invalid").unwrap(), Position::Unknown);
    }

    #[test]
    fn test_position_abbreviation() {
        assert_eq!(Position::Pitcher.abbreviation(), "P");
        assert_eq!(Position::Shortstop.abbreviation(), "SS");
        assert_eq!(Position::Unknown.abbreviation(), "??");
    }

    #[test]
    fn test_batting_side() {
        assert_eq!(BattingSide::from_str("L").unwrap(), BattingSide::Left);
        assert_eq!(BattingSide::from_str("RIGHT").unwrap(), BattingSide::Right);
        assert_eq!(BattingSide::from_str("Switch").unwrap(), BattingSide::Switch);
        assert_eq!(BattingSide::from_str("invalid").unwrap(), BattingSide::Unknown);
    }

    #[test]
    fn test_throwing_side() {
        assert_eq!(ThrowingSide::from_str("L").unwrap(), ThrowingSide::Left);
        assert_eq!(ThrowingSide::from_str("RIGHT").unwrap(), ThrowingSide::Right);
        assert_eq!(ThrowingSide::from_str("invalid").unwrap(), ThrowingSide::Unknown);
    }

    #[test]
    fn test_record() {
        let record = Record::new(42, 34);
        assert_eq!(record.to_string(), "42-34");
        assert!((record.winning_percentage() - 0.553).abs() < 0.001);

        assert!(Record::from_str("42-34").is_ok());
        assert!(Record::from_str("invalid").is_err());
    }

    #[test]
    fn test_uniform_number() {
        assert!(UniformNumber::new("42").is_ok());
        assert!(UniformNumber::new("ABC").is_err());
        
        let number = UniformNumber::new("27").unwrap();
        assert_eq!(number.to_string(), "#27");
    }
} 