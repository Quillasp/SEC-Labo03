use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Validator;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Password(String);

impl std::ops::Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
pub struct PasswordError;

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Validator::validate_password(s) {
            Ok(_) => Ok(Self(s.to_string())),
            Err(_) => Err(PasswordError),
        }
    }
}

impl Default for Password {
    fn default() -> Self {
        Password("12*#abCD".to_string())
    }
}
