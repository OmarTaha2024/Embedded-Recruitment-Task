mod server;
mod message;

use crate::server::Server;

fn main() {
    env_logger::init();

    let server = Server::new("127.0.0.1:8081").expect("Failed to start server");
    server.run().expect("Server encountered an error");
}
