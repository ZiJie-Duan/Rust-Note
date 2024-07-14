use std::{io::{stdin, stdout, Read, Write}, sync::mpsc::Receiver};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    Hostsys,
    Clisys,
    Clictrl,
    None
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
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
pub struct CommandPack {
    pub commandtype: CommandType,
    pub command: Command,
    pub args: Vec<String>
}

impl CommandPack {
    pub fn new() -> Self {
        Self {
            commandtype: CommandType::None,
            command: Command::None,
            args: Vec::new()
        }
    }

    pub fn into_string(line: String) -> CommandPack {
        let parts = line.trim_end().split(' ');
        let mut cp = CommandPack::new();
    
        for (index, part) in parts.enumerate() {
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
                                    .to_string()
                                    .replace('@', " ");
                    cp.args.push(arg);
                }
            }
        }
        cp
    }
}

pub fn get_user_input() -> String {
    let mut user_in = String::new();
    stdin().read_line(&mut user_in).expect("read_line from stdin err");
    user_in
}
