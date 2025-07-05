use sqlx::Type;
use std::fmt;

pub type PublicKeyType = String;
pub type SignatureType = String;

/// Direction of indexing (OLD or NEW)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "indexer_direction_type", rename_all = "UPPERCASE")]
pub enum Direction {
    /// Old direction
    DOWN,
    /// New direction
    UP,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::DOWN => write!(f, "DOWN"),
            Direction::UP => write!(f, "UP"),
        }
    }
}

impl From<String> for Direction {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "DOWN" => Direction::DOWN,
            "UP" => Direction::UP,
            _ => Direction::UP, // Default to NEW if the string doesn't match
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "DOWN" => Direction::DOWN,
            "UP" => Direction::UP,
            _ => Direction::UP, // Default to NEW if the string doesn't match
        }
    }
}

impl From<Direction> for String {
    fn from(d: Direction) -> Self {
        match d {
            Direction::DOWN => "DOWN".to_string(),
            Direction::UP => "UP".to_string(),
        }
    }
}
