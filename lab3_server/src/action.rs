/// This file is used to execute the various actions submitted by the clients
///
/// Tasks todo: - Improve the authentication & access controls
///             - Input/output validation
///             - Log stuff whenever required
///             - Potential improvements
use crate::access_control::{AccessController, AccessObject, Request};
use crate::connection::Connection;
use crate::database::Database;
use crate::user::{UserAccount, UserRole};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use strum_macros::{EnumIter, EnumString};

use utils::ErrorMessage;
use validation::Validator;

#[derive(Serialize, Deserialize, Debug, EnumString, EnumIter)]
pub enum Action {
    #[strum(serialize = "Show users", serialize = "1")]
    ShowUsers,
    #[strum(serialize = "Change my phone number", serialize = "2")]
    ChangeOwnPhone,
    #[strum(serialize = "Show someone's phone number", serialize = "3")]
    ChangePhone,
    #[strum(serialize = "Add user", serialize = "4")]
    AddUser,
    #[strum(serialize = "Login", serialize = "5")]
    Login,
    #[strum(serialize = "Logout", serialize = "6")]
    Logout,
    #[strum(serialize = "Exit", serialize = "7")]
    Exit,
}

/// The individual actions are implemented with three main steps:
///     1. Read client inputs if required
///     2. Execute various server code
///     3. Send a result
impl Action {
    pub fn perform(&self, u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        let res = match self {
            Action::ShowUsers => Action::show_users(u),
            Action::ChangeOwnPhone => Action::change_own_phone(u),
            Action::ChangePhone => Action::change_target_phone(u),
            Action::AddUser => Action::add_user(u),
            Action::Login => Action::login(u),
            Action::Logout => Action::logout(u),
            Action::Exit => Err("Client disconnected")?,
        };

        res
    }

    pub fn show_users(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Show users");
        let users = Database::values()?;
        let res: Result<Vec<UserAccount>, &str> = Ok(users);
        u.conn().send(&res)
    }

    pub fn change_own_phone(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Change own phone");
        let phone = u.conn().receive::<String>()?;

        // Check permissions
        if u.is_anonymous() {
            warn!("Anonymous tried to change phone number");
            u.conn()
                .send::<Result<(), ErrorMessage>>(&Err(ErrorMessage::ErrorNotAuthorized))
        } else {
            let mut user_account = u.user_account()?;
            Self::change_phone(
                u,
                &mut user_account,
                phone,
                Some(AccessObject::ChangeOwnPhone),
            )
        }
    }

    pub fn change_target_phone(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Change target phone");
        let target = u.conn().receive::<String>()?;
        let phone = u.conn().receive::<String>()?;

        if let Err(e) = Validator::validate_username(target.as_str()) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(e));
        }

        match Database::get(&target)? {
            Some(mut t) => Self::change_phone(u, &mut t, phone, None),
            None => {
                warn!("User {} not found", target);
                u.conn()
                    .send::<Result<(), ErrorMessage>>(&Err(ErrorMessage::ErrorUserNotFound))
            }
        }
    }

    pub fn change_phone(
        u: &mut ConnectedUser,
        target: &mut UserAccount,
        phone: String,
        object: Option<AccessObject>,
    ) -> Result<(), Box<dyn Error>> {
        trace!("Change phone");
        // Receive data

        if let Err(e) = Validator::validate_phone_number(phone.as_str()) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(e));
        }

        let object = object.unwrap_or(AccessObject::ChangePhone);

        // Check permissions
        let current_user = u.user_account()?;
        let perm = u.ac.enforce(Request::new(&current_user, object))?;
        let res = if perm {
            info!("Changing phone number for {}", target.username());
            target.set_phone_number(phone);
            Database::insert(&target)?;
            Ok(())
        } else {
            warn!(
                "{} tried to change phone number for {}",
                current_user.username(),
                target.username()
            );
            Err(ErrorMessage::ErrorNotAuthorized)
        };

        u.conn().send(&res)
    }

    pub fn add_user(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Add user");
        // Receive data
        let username = u.conn().receive::<String>()?;
        let password = u.conn().receive::<String>()?;
        let phone = u.conn().receive::<String>()?;
        let role = u.conn().receive::<UserRole>()?;

        if let Err(e) = Validator::validate_username(&username) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(e));
        } else if let Err(e) = Validator::validate_password(&password) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(e));
        } else if let Err(e) = Validator::validate_phone_number(&phone) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(e));
        }

        let current_user = u.user_account()?;

        let res = match u
            .ac
            .enforce(Request::new(&current_user, AccessObject::AddUser))?
        {
            true => {
                if Database::get(&username)?.is_some() {
                    warn!("User {} already exists", username);
                    Err(ErrorMessage::ErrorUserAlreadyExists)
                } else {
                    info!("Adding user {}", username);
                    let user = UserAccount::new(username, password, phone, role);
                    Database::insert(&user)?;
                    Ok(())
                }
            }
            false => Err(ErrorMessage::ErrorNotAuthorized),
        };

        u.conn.send(&res)
    }

    pub fn login(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Login");
        // Receive data
        let username = u.conn().receive::<String>()?;
        let password = u.conn().receive::<String>()?;

        if let Err(s) = Validator::validate_username(&username) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(s));
        } else if let Err(s) = Validator::validate_password(&password) {
            return u.conn().send::<Result<(), ErrorMessage>>(&Err(s));
        }

        let res = if !u.is_anonymous() {
            info!("User already logged in");
            Err(ErrorMessage::ErrorIsLoggedIn)
        } else {
            let user = Database::get(&username)?;
            if let Some(user) = user {
                if user.password() == password {
                    u.set_username(&username);
                    info!("User {} logged in", username);
                    Ok(())
                } else {
                    warn!("Wrong password for user {}", username);
                    Err(ErrorMessage::ErrorLogin)
                }
            } else {
                warn!("User {} not found", username);
                Err(ErrorMessage::ErrorLogin)
            }
        };

        u.conn.send(&res)
    }

    pub fn logout(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
        trace!("Logout");
        let res: Result<(), ErrorMessage>;

        // Check permissions
        res = if u.is_anonymous() {
            debug!("User not logged in");
            Err(ErrorMessage::ErrorNotLoggedIn)
        } else {
            info!("User {} logged out", u.username());
            u.logout();
            Ok(())
        };

        u.conn.send(&res)
    }
}

/// Used to represent a connected user for the actions
pub struct ConnectedUser {
    pub username: Option<String>,
    ac: Arc<AccessController>,
    pub conn: Connection,
}

impl ConnectedUser {
    pub fn anonymous(ac: Arc<AccessController>, conn: Connection) -> ConnectedUser {
        ConnectedUser {
            username: None,
            ac,
            conn,
        }
    }

    pub fn username(&mut self) -> String {
        self.username.as_ref().unwrap().clone()
    }

    pub fn conn(&mut self) -> &mut Connection {
        &mut self.conn
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = Some(username.to_string());
    }

    pub fn is_anonymous(&self) -> bool {
        return self.username.is_none();
    }

    pub fn logout(&mut self) {
        self.username = None;
    }

    pub fn user_account(&mut self) -> Result<UserAccount, Box<dyn Error>> {
        if self.is_anonymous() {
            // Je n'arrive pas à faire plus gracieux…
            Err(ErrorMessage::ErrorNotLoggedIn.into())
        } else {
            Ok(Database::get(&self.username())?.expect("User logged in but not in DB"))
        }
    }
}
