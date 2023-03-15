use nt::{CallbackType, ConnectionCallbackType, NetworkTables, EntryData, EntryValue};

pub async fn listen(hostname: &str, ip: &str) {
    let mut nt = NetworkTables::bind(ip, hostname);

    nt.add_connection_callback(ConnectionCallbackType::ClientConnected, |addr| {
        println!("Client connected! {}", addr);
    });

    nt.add_connection_callback(ConnectionCallbackType::ClientDisconnected, |addr| {
        println!("Client disconnected {}", addr);
    });

    nt.add_callback(CallbackType::Add, |data| {
        println!("Received new entry {:?}", data);
    });
}

pub async fn write(hostname: &str, ip: &str, entry_name: &str, entry_value: EntryValue) -> Result<(), ()> {
    let client = NetworkTables::connect(ip, hostname).await.unwrap();

    let _id = client
        .create_entry(EntryData::new(
            entry_name.to_string(),
            0,
            entry_value
        ))
        .await;

    for (id, value) in client.entries() {
        println!("{} ==> {:?}", id, value);
    }

    Ok(())
}
