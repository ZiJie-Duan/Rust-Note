use rand::Rng;
use std::io;
use std::io::{stdout, Write};

#[derive(PartialEq, Debug)]
enum GameOption {
    Rock,
    Paper,
    Scissors,
}

fn read_user_in() -> String {
    let mut uin: String = String::new();
    io::stdin()
        .read_line(&mut uin)
        .expect("failed to read line");
    return uin;
}

fn gain_option() -> GameOption {
    println!("Please Enter Your Option");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors");
    print!(">>");
    stdout().flush().unwrap();

    let choose = read_user_in();

    let result: GameOption = match choose.trim() {
        "1" => GameOption::Rock,
        "2" => GameOption::Paper,
        "3" => GameOption::Scissors,
        _ => {
            println!("You choose wrong number!");
            GameOption::Rock
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
    match first_player {
        1 => println!("Player 1 First! Player 2 Can Talk Before Palying!"),
        2 => println!("Player 2 First! Player 1 Can Talk Before Palying!"),
        _ => println!("invalid"),
    }

    let p1: String;
    let p2: String;
    if first_player == 1 {
        p1 = "Player 1".to_string();
        p2 = "Player 2".to_string();
    } else {
        p1 = "Player 2".to_string();
        p2 = "Player 1".to_string();
    }

    println!("{}", "\n".to_string() + p2.as_str() + " Please Talk First!");
    println!("Press Enter to Continue!");

    read_user_in();

    println!("\x1B[2J\x1B[1;1H");
    println!("{}", p1.clone() + " Option");
    let p1_result = gain_option();
    println!("Finish, Please Press Enter to Hide Info");

    read_user_in();

    println!("\x1B[2J\x1B[1;1H");
    println!("{}", p2.clone() + " Option");
    let p2_result = gain_option();

    println!("Finish, Please Press Enter to Hide Info");

    read_user_in();
    println!("\x1B[2J\x1B[1;1H");

    println!("Final Scope! Player1 and Player2!!!");
    println!("Press Enter to get result!");

    read_user_in();
    println!("\x1B[2J\x1B[1;1H");

    match (&p1_result, &p2_result) {
        (GameOption::Rock, GameOption::Rock) => {
            println!("Draw! No One Win!");
        }
        (GameOption::Paper, GameOption::Paper) => {
            println!("Draw! No One Win!");
        }
        (GameOption::Scissors, GameOption::Scissors) => {
            println!("Draw! No One Win!");
        }
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
    }
    println!("\n Result: ");
    println!("{} {:?}", p1.clone() + " Option: ", p1_result);
    println!("{} {:?}", p2.clone() + " Option: ", p2_result);
}
