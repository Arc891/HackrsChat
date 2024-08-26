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

    #[tokio::test]
    async fn add_user() {
        let db = db::Database::new(dotenv::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
        assert!(db.check_user_exists("test").await.unwrap() == false, "User already exists in database.");
        let user = db::User::new("test".to_string(), "test".to_string());
        db.create_user(user).await.unwrap();
    }

    #[tokio::test]
    async fn check_and_get_user() {
        let db = db::Database::new(dotenv::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
        let exists = db.check_user_exists("test").await.unwrap();
        assert_eq!(exists, true, "User does not exist in database.");
        let db_user = db.get_user_by_username("test").await.unwrap();
        assert_eq!(db_user.username, "test", "Usernames do not match.");
        assert_eq!(db_user.password_hash, "test", "Password hashes do not match.");
        assert_eq!(db_user.status, db::UserStatus::Offline, "User status is not offline.");
        assert_eq!(db_user.bio, None, "User bio is not None.");
    }

    #[tokio::test]
    async fn delete_user() {
        let db = db::Database::new(dotenv::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
        let db_user = db.get_user_by_username("test").await.unwrap();
        db.delete_user(db_user).await.unwrap();
    }

}