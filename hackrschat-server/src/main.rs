use anyhow::{Context, Ok, Result};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};
mod db;
use db::{Database, User, UserClient};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:8080")
        .await
        .context("Failed to bind.")?;
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.context("Failed to accept.")?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            println!("Accepted connection from: {}", addr);

            let (read, mut writer) = socket.split();

            let mut reader = BufReader::new(read);
            let mut line = String::new();

            let db = Database::new(dotenv::var("DATABASE_URL").unwrap().as_str())
                .await
                .context("Failed to connect to database.")?;

            println!("Entering loop...");

            loop {
                println!("Waiting for a message...");
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.is_err() {
                            println!("Failed to read from socket: {:?}", result.err());
                            break;
                        }
                        if result.unwrap() == 0 {
                            break;
                        }

                        let cmd = line.trim();
                        if cmd == "exit" { break; }
                        let response = handle_db_requests(&db, cmd).await.context("Failed to handle db requests.")?;

                        println!("Received: '{}' from {}", cmd, addr);

                        tx.send((response, addr)).context("Failed to send message")?;
                        line.clear();
                    }
                    result = rx.recv() => {
                        if result.is_err() { break; }

                        let (msg, _recv_addr) = result.unwrap();
                        // if addr == recv_addr {
                        println!("Sending: {} to {}", msg, addr);
                        writer.write_all(&msg.as_bytes()).await.context("Failed to write buf on sock")?;
                        // }
                    }
                }
            }
            println!("Connection closed.");
            Ok(())
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn handle_db_requests(db: &Database, cmd: &str) -> Result<String> {
    let ret = match cmd {
        "add_user" => "Please enter a user to add.\n".to_string(),
        cmd if cmd.starts_with("add_user ") => {
            let username = cmd[9..].to_string();
            if username.is_empty() {
                return Ok("Username cannot be empty.\n".to_string());
            }

            let user = User::new(username, "test".to_string());
            db.create_user(&user).await?;
            format!("User added: {:?}\n", user)
        }
        "get_user" => "Please enter a username to get.\n".to_string(),
        cmd if cmd.starts_with("get_user ") => {
            let username = cmd[9..].to_string();
            if username.is_empty() {
                return Ok("Username cannot be empty.\n".to_string());
            }

            if db.check_user_exists(username.as_str()).await? == false {
                return Ok("User does not exist.\n".to_string());
            }
            let user = db.get_user_by_username(&username).await?;
            let user_client = user.into_user_client();
            format!("{}\n", serde_json::to_string(&user_client).unwrap())
        }
        cmd if cmd.starts_with("get_users") => {
            if cmd.len() > 9 {
                return Ok("Invalid command, did you mean 'get_users'?.\n".to_string());
            }
            let users = db.get_users().await?;
            let users: Vec<UserClient> = users.into_iter().map(|user| user.into_user_client()).collect();
            format!("{}\n", serde_json::to_string(&users).unwrap())
        }
        "check_user" => "Please enter a username to check.\n".to_string(),
        cmd if cmd.starts_with("check_user ") => {
            let username = cmd[11..].to_string();
            if username.is_empty() {
                return Ok("Username cannot be empty.\n".to_string());
            }

            format!("{}\n", db.check_user_exists(username.as_str()).await?)
        }
        _ => {
            format!("Unknown command: {}\n", cmd)
        }
    };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::UserStatus;

    async fn setup() -> Result<Database> {
        let db_url = dotenv::var("DATABASE_URL").unwrap();
        let db = Database::new(&db_url).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn add_and_check_and_delete_user() {
        let setup = setup().await.unwrap();
        let db = setup;
        assert!(db.check_user_exists("test").await.unwrap() == false, "User already exists in database.");

        let user = User::new("test".to_string(), "test".to_string());
        db.create_user(&user).await.unwrap();
        assert!(db.check_user_exists("test").await.unwrap() == true, "User does not exist in database.");

        let db_user = db.get_user_by_username("test").await.unwrap();
        assert_ne!(db_user.id, -1, "User id is -1.");
        assert_eq!(db_user.username, "test", "Usernames do not match.");
        assert_eq!(db_user.password_hash, "test", "Password hashes do not match.");
        assert_eq!(db_user.created_at, db_user.last_online, "User created_at and last_online do not match.");
        assert_eq!(db_user.status, UserStatus::Offline, "User status is not offline.");
        assert_eq!(db_user.bio, None, "User bio is not None.");

        db.delete_user(db_user).await.unwrap();
        assert!(db.check_user_exists("test").await.unwrap() == false, "User still exists in database.");
    }
}
