use serde::{Deserialize, Serialize};

use crate::types::{UserInfo, UserStatus};

pub const PROTOCOL_VERSION: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Request {
    CheckUser { username: String },
    Register { username: String, password: String },
    Login { username: String, password: String },
    GetUsers,
    GetUser { username: String },
    SendMessage { recipient: String, content: String },
    GetMessages { peer: String },
    SetStatus { status: UserStatus },
    Disconnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    Ok,
    Error { code: ErrorCode, message: String },
    LoginSuccess { token: String },
    RegisterSuccess,
    UserExists { exists: bool },
    UserInfo(UserInfo),
    UserList(Vec<UserInfo>),
    MessageReceived { from: String, content: String, timestamp: String },
    MessageHistory { messages: Vec<MessageInfo> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    InvalidCredentials,
    UsernameTaken,
    UserNotFound,
    NotAuthenticated,
    InvalidRequest,
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}
