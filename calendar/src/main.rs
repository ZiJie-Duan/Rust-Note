use chrono::prelude::*;
use chrono::TimeDelta;
use chrono::{DateTime, Local, Utc};
use chrono::{NaiveDate, NaiveDateTime};
use std::env;
use std::env::args;
use std::process::exit;

fn print_err(source: &str, msg: &str) {
    println!("{} Error: {}", source, msg);
}

struct ArgsCommand {
    args: Vec<String>,
    index: usize,
    error: bool,
}

impl ArgsCommand {
    fn peep_next(&self) -> &str {
        match self.args.get(self.index) {
            Some(v) => return v,
            None => {
                print_err("ArgsCommand", "Loss of Command Or Argument");
                exit(0)
            }
        };
    }

    fn get_next(&mut self) -> &str {
        let arg = match self.args.get(self.index) {
            Some(v) => return v,
            None => {
                print_err("ArgsCommand", "Loss of Command Or Argument");
                exit(0)
            }
        };
        self.index += 1;
        return arg;
    }

    fn go_next(&mut self) {
        self.index += 1;
    }

    fn go_back(&mut self) {
        self.index -= 1;
    }
}

struct Event {
    st: DateTime<Utc>,
    ed: DateTime<Utc>,
    tt: String,
    dt: String,
}

impl Event {
    fn new() -> Event {
        Event {
            st: Utc::now(),
            ed: Utc::now(),
            tt: String::new(),
            dt: String::new(),
        }
    }

    fn sub_st(&mut self, argcmd: &mut ArgsCommand) {
        println!("into st");
        match argcmd.peep_next() {
            "-ymdhm" => {
                argcmd.go_next();
                let localDateTime = Local
                    .from_local_datetime(
                        &NaiveDate::from_ymd_opt(
                            argcmd.get_next().parse::<i32>().unwrap(),
                            argcmd.get_next().parse::<u32>().unwrap(),
                            argcmd.get_next().parse::<u32>().unwrap(),
                        )
                        .unwrap()
                        .and_hms_opt(
                            argcmd.get_next().parse::<u32>().unwrap(),
                            argcmd.get_next().parse::<u32>().unwrap(),
                            0,
                        )
                        .unwrap(),
                    )
                    .unwrap();
                self.st = localDateTime.to_utc();
            }
            _ => {
                let msg = "No Command Found \"".to_string() + argcmd.peep_next() + "\"";
                print_err("sub_st", &msg);
            }
        }
    }
}

struct Calendar {
    events: Vec<Event>,
}

impl Calendar {
    fn new() -> Calendar {
        Calendar { events: Vec::new() }
    }

    fn add(&mut self, argcmd: &mut ArgsCommand) {
        println!("into add");
        let mut event = Event::new();

        match argcmd.peep_next() {
            "st" => {
                argcmd.go_next();
                event.sub_st(argcmd);
            }
            _ => {
                let msg = "No Command Found \"".to_string() + argcmd.peep_next() + "\"";
                print_err("Calendar", &msg);
            }
        }

        self.events.push(event);
    }
}

fn main() {
    let mut argcmd = ArgsCommand {
        args: env::args().collect(),
        index: 1,
        error: false,
    };
    let mut calendar = Calendar::new();

    match argcmd.peep_next() {
        "add" => {
            argcmd.go_next();
            calendar.add(&mut argcmd);
        }
        _ => {
            let msg = "No Command Found \"".to_string() + argcmd.peep_next() + "\"";
            print_err("main", &msg);
        }
    }
}
