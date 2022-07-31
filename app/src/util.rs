use serde::{Deserialize, Serialize};

/// struct to contain incoming data and annotate with a session ID
#[derive(Serialize, Deserialize)]
pub struct Transmission<T> {
    session_id: u32,
    data: T,
}

impl<T> Transmission<T> {
    pub fn unwrap(self) -> (u32, T) {
        (self.session_id, self.data)
    }
}

pub struct LoginRequest {
    username: String,
    password_hash: String,
}

pub enum LoginReply {
    NoUserName,
    IncorrectPassword,
    /// Success option contains a session ID
    Success(u32),
}
