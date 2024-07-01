
use std::io;

fn main() {
    println!("斐波那契数列");
    let length:u32 = get_length();

    let mut lastf:u64 = 1;
    let mut last:u64 = 1;
    let mut temp:u64 = 0;

    let mut counter = length;

    println!("---");
    match length{
        0 => {println!("0"); return;},
        1 => {println!("1"); return;},
        2 => {println!("1\n1"); return;},
        _ => println!("1\n1"),
    };

    loop {

        if counter == 0 {
            break;
        }
        
        temp = last + lastf;
        lastf = last;
        last = temp;
        println!("{}",last);

        counter = counter - 1;
    };

    println!("FINISH");

}

fn get_length() -> u32 {

    let mut user_in : String = String::new();

    loop {
        user_in.clear();

        println!("please input the length of the array");
        io::stdin()
            .read_line(&mut user_in)
            .expect("err io");
        
        let res:u32 = match user_in.trim().parse::<u32>(){
            Ok(n) => n,
            Err(e) => {println!("err : {}", e); continue;},
        };

        return res;
    }
}
