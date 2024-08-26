use sqlx::types::time;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "userstatus")]
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

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: time::OffsetDateTime,
    pub last_online: time::OffsetDateTime,
    pub status: UserStatus,
    pub bio: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserLogin {
    id: i32,
    username: String,
    password_hash: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserProfile {
    id: i32,
    username: String,
    bio: Option<String>,
    created_at: time::OffsetDateTime,
    last_online: time::OffsetDateTime,
    status: UserStatus,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: -1,
            username,
            password_hash,
            created_at: time::OffsetDateTime::now_utc(),
            last_online: time::OffsetDateTime::now_utc(),
            status: UserStatus::Offline,
            bio: None,
        }
    }
    
    pub fn get_id(&self) -> i32 {
        self.id
    }
    
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    
    pub fn get_username(&self) -> &str {
        &self.username
    }
}