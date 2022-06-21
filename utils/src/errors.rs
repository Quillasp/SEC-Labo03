use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Error {
    #[strum(serialize = "Invalid username")]
    InvalidUsername,
    #[strum(serialize = "Invalid password")]
    InvalidPassword,
    #[strum(serialize = "Invalid phone number")]
    InvalidPhoneNumber,
    #[strum(serialize = "Invalid role")]
    InvalidRole,
    // #[strum(serialize = "User already exists")]
    // UserAlreadyExists,
    // #[strum(serialize = "User does not exist")]
    // UserDoesNotExist,
    // #[strum(serialize = "User is not logged in")]
    // UserNotLoggedIn,
    // #[strum(serialize = "User is already logged in")]
    // UserAlreadyLoggedIn,
    // #[strum(serialize = "User is not authorized")]
    // UserNotAuthorized,
    // #[strum(serialize = "User is not authorized to perform this action")]
    // UserNotAuthorizedToPerformAction,
    // #[strum(serialize = "User is not authorized to perform this action on this user")]
    // UserNotAuthorizedToPerformActionOnThisUser,
    // #[strum(serialize = "User is not authorized to perform this action on this phone number")]
    // UserNotAuthorizedToPerformActionOnThisPhoneNumber,
    // #[strum(serialize = "User is not authorized to perform this action on this role")]
    // UserNotAuthorizedToPerformActionOnThisRole,
    // #[strum(serialize = "User is not authorized to perform this action on this user account")]
    // UserNotAuthorizedToPerformActionOnThisUserAccount,
    // #[strum(serialize = "User is not authorized to perform this action on this user account role")]
    // UserNotAuthorizedToPerformActionOnThisUserAccountRole,
    // #[strum(serialize = "User is not authorized to perform this action on this user account phone number")]
    // UserNotAuthorizedToPerformActionOnThisUserAccountPhoneNumber,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum ErrorMessage {
    #[strum(
        serialize = "Invalid username input. Should be between 3 and 20 characters long and only contain letters and numbers"
    )]
    InvalidUsername,
    #[strum(
        serialize = "Invalid password input. Should be between 8 and 64 characters long and contains lowercase and uppercase letters, digits and special characters"
    )]
    InvalidPassword,
    #[strum(serialize = "Invalid phone number input. Should be in the form XXX-XXX-XXXX")]
    InvalidPhoneNumber,
    #[strum(serialize = "Invalid user or password")]
    ErrorLogin,
    #[strum(serialize = "You are already logged in")]
    ErrorIsLoggedIn,
    #[strum(serialize = "You are not logged in")]
    ErrorNotLoggedIn,
    #[strum(serialize = "You are not authorized to perform this action")]
    ErrorNotAuthorized,
    #[strum(serialize = "Target not found")]
    ErrorUserNotFound,
    #[strum(serialize = "User already exists")]
    ErrorUserAlreadyExists,
}

impl std::error::Error for ErrorMessage {}
