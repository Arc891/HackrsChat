use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};
mod db;
use db::Database;
use hackrschat_common::{
    codec,
    protocol::{ErrorCode, Request, Response},
};

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .context("Failed to connect to database.")?;
    println!("Connected to database.");

    let listener = TcpListener::bind("localhost:8080")
        .await
        .context("Failed to bind.")?;
    println!("Listening on localhost:8080");

    loop {
        let (socket, addr) = listener.accept().await.context("Failed to accept.")?;
        let db = db.clone();

        tokio::spawn(async move {
            println!("Accepted connection from: {}", addr);

            let (read, mut writer) = socket.into_split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            let mut clean_disconnect = false;

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Err(e) => {
                        eprintln!("Read error from {}: {}", addr, e);
                        break;
                    }
                    Ok(_) => {}
                }

                let request = match codec::decode_request(line.trim()) {
                    Ok(req) => req,
                    Err(e) => {
                        let err = Response::Error {
                            code: ErrorCode::InvalidRequest,
                            message: e.to_string(),
                        };
                        if let Ok(data) = codec::encode_response(&err) {
                            let _ = writer.write_all(data.as_bytes()).await;
                        }
                        continue;
                    }
                };

                if matches!(request, Request::Disconnect) {
                    if let Ok(data) = codec::encode_response(&Response::Ok) {
                        let _ = writer.write_all(data.as_bytes()).await;
                    }
                    clean_disconnect = true;
                    break;
                }

                let response = handle_request(&db, request).await;
                let encoded = match codec::encode_response(&response) {
                    Ok(data) => data,
                    Err(_) => break,
                };
                if writer.write_all(encoded.as_bytes()).await.is_err() {
                    break;
                }
            }

            if clean_disconnect {
                println!("Client disconnected cleanly: {}", addr);
            } else {
                println!("Connection lost: {}", addr);
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn handle_request(db: &Database, request: Request) -> Response {
    match request {
        Request::CheckUser { username } => match db.check_user_exists(&username).await {
            Ok(exists) => Response::UserExists { exists },
            Err(e) => Response::Error {
                code: ErrorCode::InternalError,
                message: e.to_string(),
            },
        },
        Request::GetUser { username } => match db.get_user_by_username(&username).await {
            Ok(user) => Response::UserInfo(user.into_user_info()),
            Err(e) => {
                if matches!(e, sqlx::Error::RowNotFound) {
                    Response::Error {
                        code: ErrorCode::UserNotFound,
                        message: format!("User '{}' not found", username),
                    }
                } else {
                    Response::Error {
                        code: ErrorCode::InternalError,
                        message: e.to_string(),
                    }
                }
            }
        },
        Request::GetUsers => match db.get_users().await {
            Ok(users) => {
                let user_infos = users.into_iter().map(|u| u.into_user_info()).collect();
                Response::UserList(user_infos)
            }
            Err(e) => Response::Error {
                code: ErrorCode::InternalError,
                message: e.to_string(),
            },
        },
        Request::Register { .. } => Response::RegisterSuccess, // stub — auth phase will add real registration
        Request::Login { .. } => Response::Error {
            code: ErrorCode::NotAuthenticated,
            message: "Not yet implemented".to_string(),
        },
        Request::SendMessage { .. } | Request::GetMessages { .. } => Response::Error {
            code: ErrorCode::NotAuthenticated,
            message: "Not yet implemented".to_string(),
        },
        // Disconnect is handled in the connection loop; this arm satisfies exhaustive match
        Request::SetStatus { .. } | Request::Disconnect => Response::Ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::{User, UserStatus};

    async fn setup() -> Result<Database> {
        let db_url = dotenvy::var("DATABASE_URL").unwrap();
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
