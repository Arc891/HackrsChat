use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader }, 
    net::TcpListener,
    sync::broadcast,
};
use anyhow::{ Context, Ok, Result };
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:8080").await.context("Failed to bind.")?;
    let (tx, _rx) = broadcast::channel(10);
    let _db = db::Database::new(
        dotenv::var("DATABASE_URL")
        .unwrap()
        .as_str()
    ).await.context("Failed to connect to database.")?;
    
    loop {
        let (mut socket, addr) = listener.accept().await.context("Failed to accept.")?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut writer) = socket.split();
        
            let mut reader = BufReader::new(read);
            let mut line = String::new();
        
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.is_err() || result.unwrap() == 0 { break; } 

                        tx.send((line.clone(), addr)).context("Failed to send message")?;
                        line.clear();
                    }
                    result = rx.recv() => {
                        if result.is_err() { break; }
                        
                        let (msg, recv_addr) = result.unwrap();
                        if addr != recv_addr {
                            writer.write_all(&msg.as_bytes()).await.context("Failed to write buf on sock")?;
                        }
                    }
                }
            }
            Ok(())
        });
    }
    
    #[allow(unreachable_code)]
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> Result<db::Database> {
        let db_url = dotenv::var("DATABASE_URL").unwrap();
        let db = db::Database::new(&db_url).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn add_and_check_and_delete_user() {
        let db = setup().await.unwrap();
        assert!(db.check_user_exists("test").await.unwrap() == false, "User already exists in database.");

        let user = db::User::new("test".to_string(), "test".to_string());
        db.create_user(&user).await.unwrap();
        assert!(db.check_user_exists("test").await.unwrap() == true, "User does not exist in database.");

        let db_user = db.get_user_by_username("test").await.unwrap();
        assert_ne!(db_user.id, -1, "User id is -1.");
        assert_eq!(db_user.username, "test", "Usernames do not match.");
        assert_eq!(db_user.password_hash, "test", "Password hashes do not match.");
        assert_eq!(db_user.created_at, db_user.last_online, "User created_at and last_online do not match.");
        assert_eq!(db_user.status, db::UserStatus::Offline, "User status is not offline.");
        assert_eq!(db_user.bio, None, "User bio is not None.");

        db.delete_user(db_user).await.unwrap();
        assert!(db.check_user_exists("test").await.unwrap() == false, "User still exists in database.");

    }

}