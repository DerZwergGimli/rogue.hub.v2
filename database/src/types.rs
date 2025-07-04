use sqlx::Type;
use std::fmt;

pub type PublicKeyType = String;
pub type SignatureType = String;

/// Direction of indexing (OLD or NEW)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "direction_type", rename_all = "UPPERCASE")]
pub enum Direction {
    /// Old direction
    Old,
    /// New direction
    New,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Old => write!(f, "OLD"),
            Direction::New => write!(f, "NEW"),
        }
    }
}

impl From<String> for Direction {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "OLD" => Direction::Old,
            "NEW" => Direction::New,
            _ => Direction::New, // Default to NEW if the string doesn't match
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "OLD" => Direction::Old,
            "NEW" => Direction::New,
            _ => Direction::New, // Default to NEW if the string doesn't match
        }
    }
}

impl From<Direction> for String {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Old => "OLD".to_string(),
            Direction::New => "NEW".to_string(),
        }
    }
}
