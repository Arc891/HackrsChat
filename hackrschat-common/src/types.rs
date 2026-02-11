use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "userstatus"))]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub rank: String,
    pub last_online: String,
    pub status: UserStatus,
    pub bio: Option<String>,
}

impl UserInfo {
    pub fn display_info(&self) -> String {
        format!(
            "Username: {}\nRank: {}\nLast Online: {}\nStatus: {}\nBio: {}\n",
            self.username,
            self.rank,
            self.last_online,
            self.status,
            self.bio.as_deref().unwrap_or(""),
        )
    }
}
