
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    AlreadyExists(String),
    Internal(String),
    Unauthorized
}