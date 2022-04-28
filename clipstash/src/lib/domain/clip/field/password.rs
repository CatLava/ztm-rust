use super::ClipError;
use serder::{Deserialize, Serialize};
use std::str::FromStr;

// option is password is a optional to attach to clip stash
// user does not have to implement a password
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

// use generics with the passwords
impl Password {
    // the into trait allows for option of a password string
    // will convert it into the appropriate type for usage
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
        }
        // this is for no password, and may come in for empty string    
        None => Ok(Self(None))
        //  cliperror could be added for password functionality
        // password complexity
    }

    pub fn into_inner(self) -> Option<String>{
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

// when implementing this need err type
impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}