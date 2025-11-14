use net_worm::server::Server;

fn main() {
    let mut server = Server::new();

    server.server_loop();
}