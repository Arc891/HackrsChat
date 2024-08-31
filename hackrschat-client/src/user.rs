use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Online,
    Away,
    Offline,
}

impl From<()> for UserStatus {
    fn from(_: ()) -> Self {
        UserStatus::Offline
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Away => write!(f, "Away"),
            UserStatus::Online => write!(f, "Online"),
            UserStatus::Offline => write!(f, "Offline"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    username: String,
    rank: String,
    last_online: String,
    status: UserStatus,
    bio: Option<String>,
}

impl User {
    pub fn new(username: String, rank: String, last_online: String, status: UserStatus, bio: Option<String>) -> Self {
        Self {
            username,
            rank,
            last_online,
            status,
            bio,
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_rank(&self) -> String {
        self.rank.clone()
    }

    pub fn get_last_online(&self) -> String {
        self.last_online.clone()
    }

    pub fn get_status(&self) -> UserStatus {
        self.status.clone()
    }

    pub fn get_bio(&self) -> String {
        match &self.bio {
            Some(bio) => bio.clone(),
            None => "".to_string(),
        }
    }

    pub fn display_info(&self) -> String {
        format!("Username: {}\nRank: {}\nLast Online: {}\nStatus: {}\nBio: {}\n", 
            self.get_username(), 
            self.get_rank(), 
            self.get_last_online(), 
            self.get_status(), 
            self.get_bio()
        )
    }
}
