use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Validator;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct PhoneNumber(String);

impl std::ops::Deref for PhoneNumber {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct PhoneNumberError;

impl FromStr for PhoneNumber {
    type Err = PhoneNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Validator::validate_phone_number(s) {
            return Ok(PhoneNumber(String::from(s)));
        } else {
            return Err(PhoneNumberError);
        }
    }
}

impl Default for PhoneNumber {
    fn default() -> Self {
        PhoneNumber("+1-555-555-5555".to_string())
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
