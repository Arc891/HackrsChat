use serde::{Serialize, Deserialize};
use hackrschat_common::types::{UserStatus, UserInfo};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: time::OffsetDateTime,
    pub last_online: time::OffsetDateTime,
    pub status: UserStatus,
    pub bio: Option<String>,
}

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
            "Online".to_string()
        } else if duration < 60 {
            format!("Last online: {} minutes ago", duration)
        } else if duration < 1440 {
            format!("Last online: {} hours ago", duration / 60)
        } else {
            format!("Last online: {} days ago", duration / 1440)
        }
    }

    pub fn display_info(&self) -> String {
        format!(
            "User: {}\nBio: {}\nRank: {}\nLast Online: {}\nStatus: {:?}",
            self.username,
            self.bio.as_deref().unwrap_or("No bio"),
            self.creation_time_rank(),
            self.format_last_online(),
            self.status
        )
    }

    pub fn into_user_info(&self) -> UserInfo {
        UserInfo {
            username: self.username.clone(),
            rank: self.creation_time_rank().to_string(),
            last_online: self.format_last_online(),
            status: self.status,
            bio: self.bio.clone(),
        }
    }
}
