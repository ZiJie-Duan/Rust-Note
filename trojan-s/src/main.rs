use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct HeartBeat {
    id : String
}

#[derive(Serialize, Deserialize, Debug)]
enum CommandType {
    SYSTEM,
    USER
}

#[derive(Serialize, Deserialize, Debug)]
struct CommandPack {
    ctype : CommandType,
    head : String,
    args : Vec<String>
}


fn handle_client(mut stream: TcpStream) {

    let mut buffer = [0; 2048];
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
                let buf_str = String::from_utf8_lossy(&buf_vec);
                let cp: CommandPack = serde_json::from_str(&buf_str).unwrap();
                println!("{:?}", cp);
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}


fn say_hello() {
    println!("\n\nRust Trojan Server v0.0.1\n\n");
}

fn main() -> std::io::Result<()> {

    say_hello();

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Control Server Listing at Port: 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
