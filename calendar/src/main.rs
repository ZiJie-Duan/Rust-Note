use chrono::prelude::*;

struct Calendar {
    st: DateTime<Utc>,
}

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    println!("当前的日期和时间: {}", utc.format("%y-%m-%d %h:%m:%s"));
}
