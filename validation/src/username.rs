use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Validator;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Username(String);

impl std::ops::Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct UsernameError;

impl FromStr for Username {
    type Err = UsernameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Validator::validate_username(s) {
            return Ok(Username(String::from(s)));
        } else {
            return Err(UsernameError);
        }
    }
}

impl Default for Username {
    fn default() -> Self {
        Username("username".to_string())
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
