use std::io;

fn main() {
    println!("Hi WWB 你好汪文博！");
    println!("这是我基于Rust 语言构建的脚本");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取失败");
    
    println!("you type: {guess}");
    
}
