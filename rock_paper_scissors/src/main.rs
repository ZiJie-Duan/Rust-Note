use rand::Rng;
use std::io;
use std::io::{stdout, Write};

#[derive(PartialEq, Debug)]
enum GameOption {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for GameOption {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (GameOption::Rock, GameOption::Rock) => std::cmp::Ordering::Equal,
            (GameOption::Paper, GameOption::Paper) => std::cmp::Ordering::Equal,
            (GameOption::Rock, GameOption::Paper) => std::cmp::Ordering::Less,
            (GameOption::Rock, GameOption::Scissors) => std::cmp::Ordering::Greater,
            (GameOption::Paper, GameOption::Rock) => std::cmp::Ordering::Greater,
            (GameOption::Paper, GameOption::Scissors) => std::cmp::Ordering::Less,
            (GameOption::Scissors, GameOption::Rock) => std::cmp::Ordering::Less,
            (GameOption::Scissors, GameOption::Paper) => std::cmp::Ordering::Greater,
            (GameOption::Scissors, GameOption::Scissors) => std::cmp::Ordering::Equal,
        })
    }
}

fn read_user_in() -> String {
    let mut uin = String::new();
    io::stdin()
        .read_line(&mut uin)
        .expect("failed to read line");
    uin
}

fn gain_option() -> GameOption {
    println!("Please Enter Your Option");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors");
    print!(">>");
    stdout().flush().unwrap();

    let result: GameOption = loop {
        let choose = read_user_in();
        match choose.trim() {
            "1" => break GameOption::Rock,
            "2" => break GameOption::Paper,
            "3" => break GameOption::Scissors,
            _ => eprintln!("You choose wrong number! Please choose again!"),
        }
    };
    result
}

fn main() {
    println!("\n\nRock, Paper, Scissors! Super Giao V1");
    println!("Are You both too ready?!");
    println!("Press Enter to Start!");
    read_user_in();
    let first_player = rand::thread_rng().gen_range(1..=2);

    println!(
        "Player {first_player} First! Player {} Can Talk Before Playing!",
        first_player % 2 + 1
    );

    let p1 = format!("Player {first_player}");
    let p2 = format!("Player {}", first_player % 2 + 1);
    /*if first_player == 1 {
        p1 = "Player 1".to_string();
        p2 = "Player 2".to_string();
    } else {
        p1 = "Player 2".to_string();
        p2 = "Player 1".to_string();
    }*/

    println!("{p2} please Talk First!");
    println!("Press Enter to Continue!");

    read_user_in();

    println!("\x1B[2J\x1B[1;1H");
    println!("{p1} Option");
    let p1_result = gain_option();
    println!("Finish, Please Press Enter to Hide Info");

    read_user_in();

    println!("\x1B[2J\x1B[1;1H");
    println!("{p2} Option");
    let p2_result = gain_option();

    println!("Finish, Please Press Enter to Hide Info");

    read_user_in();
    println!("\x1B[2J\x1B[1;1H");

    println!("Final Scope! Player1 and Player2!!!");
    println!("Press Enter to get result!");

    read_user_in();
    println!("\x1B[2J\x1B[1;1H");

    /*match (&p1_result, &p2_result) {
        (a, b) if a == b => {
            println!("Draw! No One Win!");
        }
        /*(GameOption::Paper, GameOption::Paper) => {
            println!("Draw! No One Win!");
        }
        (GameOption::Scissors, GameOption::Scissors) => {
            println!("Draw! No One Win!");
        }*/
        (GameOption::Rock, GameOption::Paper) => {
            println!("{} Win!", p2);
        }
        (GameOption::Paper, GameOption::Rock) => {
            println!("{} Win!", p1);
        }
        (GameOption::Scissors, GameOption::Paper) => {
            println!("{} Win!", p1);
        }
        (GameOption::Paper, GameOption::Scissors) => {
            println!("{} Win!", p2);
        }
        (GameOption::Rock, GameOption::Scissors) => {
            println!("{} Win!", p1);
        }
        (GameOption::Scissors, GameOption::Rock) => {
            println!("{} Win!", p2);
        }
        _ => {}
    }*/
    match p1_result.partial_cmp(&p2_result) {
        Some(std::cmp::Ordering::Less) => {
            println!("{} Win!", p2);
        }
        Some(std::cmp::Ordering::Greater) => {
            println!("{} Win!", p1);
        }
        Some(std::cmp::Ordering::Equal) => {
            println!("Draw! No One Win!");
        }
        None => {}
    }
    println!("\n Result: ");
    println!("{p1} Option: {p1_result:?}");
    println!("{p2} Option: {p2_result:?}");
}
