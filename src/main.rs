use std::thread;
use std::fs::File;
use std::time::Duration;
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::io::{self, ErrorKind, Read, Write};

use serde_json::{Value};

const MSG_SIZE: usize = 32;
const LOCAL: &str = "127.0.0.1:6000";

fn thread_sleep() {
    thread::sleep(Duration::from_millis(120));
}

fn main() {
    // TODO: Handle panic when package.json
    let mut file = File::open("package.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let foo: Value = serde_json::from_str(&buff).unwrap();
    println!("Scripts: {}", foo["scripts"]);

    for (name, obj) in foo["scripts"].as_object().unwrap().iter() {
        println!("{} is {:?}", name, obj);
    }

    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("Received message: {:?}", String::from_utf8(msg).unwrap());
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server lost.");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buffer = msg.clone().into_bytes();
                buffer.resize(MSG_SIZE, 0);
                client.write_all(&buffer).expect("Writing to socket failed");
                println!("Sent message: {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread_sleep();
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Reading from stdin failed.");
        let msg = buff.trim().to_string();
        if msg == ":q" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("Exited.");
}