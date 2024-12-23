use crate::message::EchoMessage;
use log::{error, info, warn};
use prost::Message;
use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::thread;

struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream }
    }

    pub fn handle(&mut self) -> io::Result<()> {
        let mut buffer = [0; 512];

        // Read data from the client
        let bytes_read = self.stream.read(&mut buffer)?;
        if bytes_read == 0 {
            info!("Client disconnected.");
            return Ok(());
        }

        match EchoMessage::decode(&buffer[..bytes_read]) {
            Ok(message) => {
                info!("Received: {}", message.content);
                // Echo back the message
                let payload = message.encode_to_vec();
                self.stream.write_all(&payload)?;
                self.stream.flush()?;
            }
            Err(e) => {
                error!("Failed to decode message: {}", e);
                return Err(io::Error::new(ErrorKind::InvalidData, "Decoding failed"));
            }
        }

        Ok(())
    }
}

pub struct Server {
    listener: TcpListener,
    is_running: Arc<AtomicBool>,
}

impl Server {
    /// Creates a new server instance
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let is_running = Arc::new(AtomicBool::new(false));
        Ok(Server {
            listener,
            is_running,
        })
    }

    /// Runs the server
    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst); // Set server as running
        info!("Server is running on {}", self.listener.local_addr()?);

        while self.is_running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr);

                    // Spawn a thread for handling the client
                    let is_running = Arc::clone(&self.is_running);
                    thread::spawn(move || {
                        let mut client = Client::new(stream);
                        while is_running.load(Ordering::SeqCst) {
                            if let Err(e) = client.handle() {
                                error!("Error handling client: {}", e);
                                break;
                            }
                        }
                        info!("Connection with {} closed.", addr);
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // Yield to reduce CPU usage
                    thread::yield_now();
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }

        info!("Server stopped.");
        Ok(())
    }

 #[allow(dead_code)]
pub fn stop(&self) {
    if self.is_running.load(Ordering::SeqCst) {
        self.is_running.store(false, Ordering::SeqCst);
        info!("Shutdown signal sent.");
    } else {
        warn!("Server was already stopped or not running.");
    }
}

}
