/// This file is used to store and retrieve user accounts from the database
///
/// Tasks todo: - Potential improvements
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display, Hash, Copy)]
pub enum UserRole {
    #[strum(serialize = "anon")]
    Anon,
    #[strum(serialize = "standard_user")]
    StandardUser,
    #[strum(serialize = "hr")]
    HR,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAccount {
    pub username: String,
    password: String,
    pub phone_number: String,
    pub role: UserRole,
}

impl UserAccount {
    pub fn new(username: String, password: String, phone_number: String, role: UserRole) -> Self {
        Self {
            username,
            password,
            phone_number,
            role,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
    }
}
