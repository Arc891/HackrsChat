use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader }, 
    net::TcpListener,
    sync::broadcast,
};
use anyhow::{ Context, Ok, Result };

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:8080").await.context("Failed to bind.")?;
    let (tx, _rx) = broadcast::channel(10);
    
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
    
    Ok(())
}




















// #[tokio::main(flavor = "multi_thread")]
// async fn main() {
//     // test_something().await;
//     let racer01 = F1Racer::new();
//     let mut racer02 = F1Racer::new();

//     racer02.lap_times.pop();
//     racer02.lap_times.push(57u8);
//     racer02.name = "Sergio Perez".to_string();

//     let handle01 = tokio::task::spawn(racer01);
//     let handle02 = tokio::task::spawn(racer02);

//     loop {
//         if handle01.is_finished() && handle02.is_finished() {
//             println!("All racers have finished!");
//             break;
//         }

//         std::thread::sleep(std::time::Duration::from_millis(300));

//     }
    
// }

// struct F1Racer {
//     name: String,
//     completed_laps: u8,
//     laps: u8,
//     best_lap_time: u8,
//     lap_times: Vec<u8>,
// }

// impl F1Racer {
//     fn new() -> F1Racer {
//         return F1Racer {
//             name: "Max Verstappen".to_string(),
//             laps: 5,
//             completed_laps: 0,
//             best_lap_time: u8::MAX,
//             lap_times: vec![87u8, 64, 126, 95, 76],
//         }
//     }

//     fn do_lap(&mut self) {
//         println!("{} is doing a new lap...", self.name);
//         let lap_time = self.lap_times.pop();
        
//         if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
//             self.best_lap_time = lap_time.unwrap();
//         }

//         self.completed_laps += 1;
//     }
// }

// impl std::future::Future for F1Racer {
//     type Output = u8;

//     fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         println!("Thread assigned is id {:?}", std::thread::current().id());
//         if self.completed_laps < self.laps {
//             self.get_mut().do_lap();
//             cx.waker().wake_by_ref();
//             return std::task::Poll::Pending;
//         }
//         println!("{} has completed all laps!", self.name);
//         println!("Best lap time for {}: {}", self.name, self.best_lap_time);

//         return std::task::Poll::Ready(self.best_lap_time);
//     }
// }

// async fn test_something() {
//         std::thread::sleep(std::time::Duration::from_millis(5000));
//         println!("Hello, world!");
// }

// use std::net::TcpListener;
// use std::net::TcpStream;
// use std::io::prelude::*;

// fn main() {
//    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

//    for stream in listener.incoming() {
//        let stream = stream.unwrap();

//        handle_connection(stream);
//    }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     stream.read(&mut buffer).unwrap();
//     println!(
//         "Request: {}",
//         String::from_utf8_lossy(&buffer[..])
//     );
// }
