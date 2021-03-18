use std::env;
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::process::exit;
use std::sync::mpsc;
use std::thread;

fn main() -> io::Result<()> {
    if env::args().len() <= 2 {
        println!("Missing parameters");
        exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let new_port = &args[1];
    let old_port = &args[2];

    let stdin = io::stdin();

    for item in stdin.lock().lines() {
        let host = format!("{}", item.unwrap());

        if check_port(&host, new_port) {
            println!(
                "Successfully connected to the host [{}] on port [{}]",
                host, new_port
            );
        } else {
            println!(
                "Failed to connect to the host [{}] on port [{}]",
                host, new_port
            );
        }

        if check_port(&host, old_port) {
            println!(
                "Successfully connected to the host [{}] on port [{}]",
                host, old_port
            );
        } else {
            println!(
                "Failed to connect to the host [{}] on port [{}]",
                host, old_port
            );
        }
    }

    Ok(())
}

fn check_port(host: &str, port: &str) -> bool {
    let host_port = format!("{}:{}", host, port);

    let (sender, receiver) = mpsc::channel();

    let t = thread::spawn(move || {
        match sender.send(TcpStream::connect(host_port)) {
            Ok(()) => {} // everything good
            Err(_) => {} // we have been released, don't panic
        }
    });

    // set time thread
    thread::sleep(std::time::Duration::new(2, 0));

    match receiver.try_recv() {
        Ok(Ok(_handle)) => true, // we have a connection
        Ok(Err(_)) => false,     // connecting failed
        Err(mpsc::TryRecvError::Empty) => {
            drop(receiver);
            drop(t);
            // connecting took more than 2 seconds
            false
        }
        Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
    }
}

fn _check_port_test(host: &String, port: &String) -> bool {
    let host_port = format!("{}:{}", host, port);

    if let Ok(_stream) = TcpStream::connect(host_port) {
        return true;
    } else {
        return false;
    }
}
