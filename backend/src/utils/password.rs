#[derive(thiserror::Error, Debug)]
pub enum PasswordError {
    #[error("Password too short. Minimum size: {} characters", PASS_MIN_LENGTH)]
    TooShort,

    #[error("Password too long. Maximum size: {} characters", PASS_MAX_LENGTH)]
    TooLong,

    #[error(
        "Password doesn't contain enough Special characters. Minimum special characters: {}",
        PASS_MIN_SYMBOLS
    )]
    NotEnoughSymbols,

    #[error("Password doesn't contain enough Digits. Minimum digits: {}", PASS_MIN_DIGITS)]
    NotEnoughDigits,

    #[error("Password doesn't contain enough Letters. Minimum letters: {}", PASS_MIN_LETTERS)]
    NotEnoughLetters,
}

const PASS_MIN_LENGTH: usize = 8;
const PASS_MAX_LENGTH: usize = 160;

const PASS_MIN_SYMBOLS: usize = 1;
const PASS_MIN_DIGITS: usize = 1;
const PASS_MIN_LETTERS: usize = 1;

const DIGITS: &str = "1234567890";
const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SPECIAL_SYMBOLS: &str = "-_/\\(){}[]|!@#$%^&*)+=\"\';:<>,.?";

fn contains_number(pass: &str, allowed: &str) -> usize {
    let mut i: usize = 0;
    for character in pass.chars() {
        if allowed.contains(character) {
            i += 1;
        }
    }
    i
}

pub fn valid_password(password: &str) -> Result<(), PasswordError> {
    if password.len() < PASS_MIN_LENGTH {
        return Err(PasswordError::TooShort);
    }
    if password.len() > PASS_MAX_LENGTH {
        return Err(PasswordError::TooLong);
    }
    if contains_number(password, SPECIAL_SYMBOLS) < PASS_MIN_SYMBOLS {
        return Err(PasswordError::NotEnoughSymbols);
    }
    if contains_number(password, DIGITS) < PASS_MIN_DIGITS {
        return Err(PasswordError::NotEnoughDigits);
    }
    if contains_number(password, LETTERS) < PASS_MIN_LETTERS {
        return Err(PasswordError::NotEnoughLetters);
    }
    Ok(())
}
