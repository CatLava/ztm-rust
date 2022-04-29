use super::ClipError;
use serde::{Deserialize, Serialize};
use derive_more::From;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, From)]
// we are going from String to shortcode
// DB is searching for the shortcode
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        // using random number generator for shortcode
        use rand::prelude::*;
        let allowed_chars = [
            'a', 'b', 'c', 'd', '1', '2', '3', '4'
        ];
        // random number generator thread function
        let mut rng = thread_rng();
        // maximum number of characters
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            );
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

// Shortcode will be used frequently and want to implement from clauses

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_string())
    }
}

// This is done for completeness
impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}