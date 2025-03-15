use std::env;
use std::sync::OnceLock;

/// Configuration for the application
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL for the MLB API
    pub mlb_api_base_url: String,
    /// Base URL for the NBA API
    pub nba_api_base_url: String,
    /// API key for the NBA API
    pub nba_api_key: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Get the application configuration
pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        // Default values
        let mlb_api_base_url = env::var("MLB_API_BASE_URL")
            .unwrap_or_else(|_| "https://statsapi.mlb.com/api/v1".to_string());
        
        let nba_api_base_url = env::var("NBA_API_BASE_URL")
            .unwrap_or_else(|_| "https://api.balldontlie.io/v1".to_string());
        
        let nba_api_key = env::var("NBA_API_KEY")
            .unwrap_or_else(|_| "".to_string());
        
        Config {
            mlb_api_base_url,
            nba_api_base_url,
            nba_api_key,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let config = get_config();
        assert!(!config.mlb_api_base_url.is_empty());
        assert!(!config.nba_api_base_url.is_empty());
        // Note: NBA API key might be empty in tests
    }
} 