use serde::{Deserialize, Serialize};

#[allow(dead_code)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: time::OffsetDateTime,
    pub last_online: time::OffsetDateTime,
    pub status: UserStatus,
    pub bio: Option<String>,
}

// #[derive(Debug, sqlx::FromRow)]
// pub struct UserLogin {
//     id: i32,
//     username: String,
//     password_hash: String,
// }

// #[derive(Debug, sqlx::FromRow)]
// pub struct UserProfile {
//     id: i32,
//     username: String,
//     bio: Option<String>,
//     created_at: time::OffsetDateTime,
//     last_online: time::OffsetDateTime,
//     status: UserStatus,
// }

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        let now = time::OffsetDateTime::now_utc();
        Self {
            id: -1,
            username,
            password_hash,
            created_at: now,
            last_online: now,
            status: UserStatus::Offline,
            bio: None,
        }
    }

    pub fn creation_time_rank(&self) -> &str {
        let now = time::OffsetDateTime::now_utc();
        let duration = (now - self.created_at).whole_days();

        if duration < 1 {
            "Newbie"
        } else if duration < 7 {
            "Script Kiddie"
        } else if duration < 14 {
            "Java Enthousiast"
        } else if duration < 30 {
            "TCP/IP Stacked"
        } else if duration < 90 {
            "Network Ninja"
        } else if duration < 365 {
            "Zero-Day Hunter"
        } else if duration < 365 * 2 {
            "Kernel Developer"
        } else {
            "Root Admin"
        }

        // Cool names to save:
        // Kernel Developer
        // Zero-Day Specialist
        // Anonymous Member - think about options

        // Could also use network layers as a reference
    }

    pub fn format_last_online(&self) -> String {
        let now = time::OffsetDateTime::now_utc();
        let duration = (now - self.last_online).whole_minutes();

        if duration < 1 {
            "Now".to_string()
        } else if duration < 60 {
            format!("{} minutes ago", duration)
        } else if duration < 1440 {
            format!("{} hours ago", duration / 60)
        } else {
            format!("{} days ago", duration / 1440)
        }
    }

    pub fn display_info(&self) -> String {
        format!(
            "User: {}, Bio: {}, Rank: {}, Status: {:?}, Last seen: {}",
            self.get_username(),
            self.get_bio().as_deref().unwrap_or("No bio"),
            self.creation_time_rank(),
            self.get_status(),
            self.format_last_online(),
        )
        .to_owned()
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_status(&self) -> UserStatus {
        self.status.clone()
    }

    pub fn get_bio(&self) -> Option<String> {
        self.bio.clone()
    }
}
