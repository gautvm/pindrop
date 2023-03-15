use nt::{CallbackType, ConnectionCallbackType, NetworkTables};

pub async fn listen(hostname: &str, ip: &str) {
    let mut nt = NetworkTables::bind(ip, hostname);

    nt.add_connection_callback(ConnectionCallbackType::ClientConnected, |addr| {
        println!("Client connected! {}", addr);
    });

    nt.add_connection_callback(ConnectionCallbackType::ClientDisconnected, |addr| {
        println!("Client disconnected {}", addr);
    });

    nt.add_callback(CallbackType::Add, |data| {
        println!("Got new entry {:?}", data);
    });
}
