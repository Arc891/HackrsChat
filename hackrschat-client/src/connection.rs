use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

use hackrschat_common::codec;
use hackrschat_common::protocol::{Request, Response};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConnectionError {
    Io(io::Error),
    Codec(serde_json::Error),
    Disconnected,
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::Io(e) => write!(f, "I/O error: {}", e),
            ConnectionError::Codec(e) => write!(f, "Codec error: {}", e),
            ConnectionError::Disconnected => write!(f, "Disconnected from server"),
        }
    }
}

pub struct ServerConnection {
    req_tx: mpsc::Sender<Request>,
    resp_rx: mpsc::Receiver<Response>,
    // push_rx will be used in the messaging phase for server-push messages
    _push_rx: mpsc::Receiver<Response>,
}

impl ServerConnection {
    pub fn connect(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let read_stream = stream.try_clone()?;

        let (req_tx, req_rx) = mpsc::channel::<Request>();
        let (resp_tx, resp_rx) = mpsc::channel::<Response>();
        // push_tx/push_rx: plumbing for future server-push messages.
        // Sender is dropped here (unused until messaging phase); receiver is stored as placeholder.
        let (_push_tx, push_rx) = mpsc::channel::<Response>();

        // Writer thread: receives Requests from the main thread and writes them to the socket.
        let mut write_stream = stream;
        thread::spawn(move || {
            while let Ok(request) = req_rx.recv() {
                let encoded = match codec::encode_request(&request) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Encode error: {}", e);
                        break;
                    }
                };
                if write_stream.write_all(encoded.as_bytes()).is_err() {
                    break;
                }
                if write_stream.flush().is_err() {
                    break;
                }
            }
        });

        // Reader thread: reads Responses from the socket and sends them to the main thread.
        // Currently all responses go to resp_tx. When server-push messages are added,
        // the reader will distinguish pushed messages and route them to push_tx instead.
        thread::spawn(move || {
            let mut reader = BufReader::new(read_stream);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        break;
                    }
                    Ok(_) => {}
                }
                let response = match codec::decode_response(line.trim()) {
                    Ok(resp) => resp,
                    Err(e) => {
                        eprintln!("Decode error: {}", e);
                        break;
                    }
                };
                if resp_tx.send(response).is_err() {
                    break;
                }
            }
        });

        Ok(Self {
            req_tx,
            resp_rx,
            _push_rx: push_rx,
        })
    }

    /// Send a request and wait for the response.
    ///
    /// Ordering invariant: request-response pairing relies on (1) only one thread
    /// calling send() at a time (Cursive is single-threaded) and (2) the server
    /// processing requests sequentially per connection. If either invariant is
    /// violated, add request IDs to the protocol.
    ///
    /// One response per request, always. The server sends exactly one Response
    /// for each Request.
    pub fn send(&self, request: &Request) -> Result<Response, ConnectionError> {
        self.req_tx
            .send(request.clone())
            .map_err(|_| ConnectionError::Disconnected)?;
        self.resp_rx
            .recv()
            .map_err(|_| ConnectionError::Disconnected)
    }
}

impl Drop for ServerConnection {
    fn drop(&mut self) {
        // Best-effort: send Disconnect before channels are dropped.
        // The server handles both clean disconnect and abrupt EOF identically.
        let _ = self.req_tx.send(Request::Disconnect);
    }
}
