
use thiserror::Error as ThisError;

pub struct UserContx {
    pub user_id: i64,
}

pub async fn utx_from_token(token: &str) -> Result<UserContx, Error> {
    match token.parse::<i64>() {
        Ok(user_id) => Ok(UserContx { user_id}),
        Err(_) => Err(Error::InvalidToken(token.to_string()))
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Ivalid Token {0}")]
    InvalidToken(String),
}