use std::sync::mpsc::Receiver;

pub fn start(incoming: Receiver<String>) {
    loop {
        let log = incoming
            .recv()
            .unwrap_or("{\"message\": \"Failed to decode logs\"}".to_string());
        println!("{}", log)
    }
}
