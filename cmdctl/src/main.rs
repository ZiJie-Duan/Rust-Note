use std::process::Command;

fn main() {
    // 创建一个命令，调用 cmd 并执行 cd 和 dir 指令
    let output = Command::new("cmd")
        .args(&["/C", "cd C:\\Users\\lucyc && dir"])
        .output()
        .expect("Failed to execute command");

    // 将命令的标准输出和标准错误输出转换为字符串并打印
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Standard Output:\n{}", stdout);
    println!("Standard Error:\n{}", stderr);
}
