use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hi Hello World!");
    println!("欢迎来到我的快乐猜数游戏");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        println!("请猜测一个数字：");

        io::stdin()
            .read_line(&mut guess)
            .expect("读取失败");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜测 : {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    
}
