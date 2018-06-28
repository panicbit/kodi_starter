extern crate psutil;

use std::net::UdpSocket;
use std::net::IpAddr;
use std::process::Command;
use std::thread;

const START_PHRASE: &[u8] = b"YatseStart";

fn main() {
    let ip = IpAddr::from([0, 0, 0, 0]);
    let port = 5600;
    let addr = (ip, port);
    let socket = UdpSocket::bind(addr).unwrap();

    loop {
        let mut data = [0; 4 * 1024];
        socket.recv_from(&mut data).ok();

        for snippet in data.windows(START_PHRASE.len()) {
            if snippet == START_PHRASE {
                start_kodi();
            }
        }
    }
}

fn start_kodi() {
    if process_list().iter().find(|ps| ps.comm == "kodi").is_some() {
        println!("Kodi is already running");
        return;
    }

    let mut child = match Command::new("kodi").spawn() {
        Ok(child) => child,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    thread::spawn(move || child.wait());
}

fn process_list() -> Vec<psutil::process::Process> {
    loop {
        match psutil::process::all() {
            Ok(procs) => return procs,
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    println!("{}", e);
                    return Vec::new();
                }
            }
        }
    }
}
