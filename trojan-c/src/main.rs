use std::net::TcpStream;
use std::io::{stdin, Read, Write};

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

fn get_user_input() -> String{
    let mut user_in = String::new();
    stdin().read_line(& mut user_in).expect("this is err");
    return user_in;
}


fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            loop{
                let cp = CommandPack {
                    ctype : CommandType::SYSTEM,
                    head : String::from("this is a head"),
                    args : vec!["arg1".into(), "arg2".into()],
                };
                
                let cp_str = serde_json::to_string(&cp).unwrap();
                let v = Vec::from(cp_str);

                stream.write(&v).unwrap();
                println!("Sent message: {:?}", to_string(cp_str));

                let mut buffer = [0 as u8; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("Reply from server: {:?}", &buffer[0..size]);
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
