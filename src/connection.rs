pub mod connection {

    pub struct Connection {
        connection_type: ConnectionType,
        addresses: (IPAddress, IPAddress),
    }

    enum ConnectionType {
        WebSocket,
        HTTPS,
        TCP,
        UDP,
    }
}