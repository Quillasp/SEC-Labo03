use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Email validation
    static ref USERNAME_RULE: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").unwrap();

    // Password validation
    static ref PW_LENGTH_RULE: Regex = Regex::new(r"^.{8,64}$").unwrap();
    static ref PW_UPPER_RULE: Regex = Regex::new(r"[[:upper:]]").unwrap();
    static ref PW_LOWER_RULE: Regex = Regex::new(r"[[:lower:]]").unwrap();
    static ref PW_DIGIT_RULE: Regex = Regex::new(r"[[:digit:]]").unwrap();
    static ref PW_SPECIAL_RULE: Regex = Regex::new(r"[#?!@$ %&\*\^\-\+\./\\]").unwrap();

    // Phone number validation
    static ref PHONE_NUMBER_RULE: Regex = Regex::new(r"^((\+|00)[0-9]{1,2}-)?[0-9]{3}-[0-9]{3}-[0-9]{4}$").unwrap();
}

pub struct Validator;

impl Validator {
    pub fn validate_username(username: &str) -> bool {
        USERNAME_RULE.is_match(username)
    }

    pub fn validate_password(password: &str) -> bool {
        PW_LENGTH_RULE.is_match(password)
            && PW_UPPER_RULE.is_match(password)
            && PW_LOWER_RULE.is_match(password)
            && PW_DIGIT_RULE.is_match(password)
            && PW_SPECIAL_RULE.is_match(password)
    }

    pub fn validate_phone_number(phone_number: &str) -> bool {
        PHONE_NUMBER_RULE.is_match(phone_number)
    }
}
