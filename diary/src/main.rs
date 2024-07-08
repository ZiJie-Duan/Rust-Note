use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let filename = r"C:\Users\lucyc\Desktop\hello.txt"; // 替换为你的文件路径
    let contents = fs::read_to_string(filename)?;

    println!("File contents:\n{}", contents);

    Ok(())
}




