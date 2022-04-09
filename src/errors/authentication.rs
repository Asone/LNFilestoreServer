#[derive(Debug)]
pub enum AuthenticationError {
    DbError(String),
    PasswordMismatch(String),
    LoginError(String),
    UserNotFound(String),
}
