
use std::thread;
use std::io::{Read, Write, Result};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};


fn handle_client(mut stream: TcpStream, tx: Sender<String>) {

    let mut buffer = [0; 128];
    // show ip addr of the client
    println!("Connection from: {}", stream.peer_addr().unwrap());

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed by the client
                println!("Connection closed");
                break;
            }
            Ok(n) => {
                // Echo the received data back to the client
                let buf_vec = Vec::from(&buffer[..n]);
                let token = String::from_utf8_lossy(&buf_vec);
                tx.send(token.to_string()).unwrap();
                stream.write(&[0 as u8]).unwrap();
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}


pub fn heart_beat(tx: Sender<String>) -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("[SOCKET]: Listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tx = tx.clone();
                thread::spawn(move|| {
                    handle_client(stream, tx);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
