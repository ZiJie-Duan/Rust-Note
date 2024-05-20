
use std::io;


fn main() {

    println!("Hello, world!");
    let mode: u32 = get_mode();

    if mode == 1{
        println!("\nCelsius to Fahrenheit");
        let temp: f32 = get_num();
        let result: f32 = (temp * 1.8) + 32.0;
        println!("Result {result}");
    } else {
        println!("\nFahrenheit to Celsius");
        let temp: f32 = get_num();
        let result: f32 = (temp * 5.0/9.0) - (32.0*5.0/9.0);
        println!("Result {result}");
    }
}

fn get_mode() -> u32 {

    let mut user_in: String = String::new();

    loop {
        user_in.clear();

        println!("Please select mode\n [1]C to F\n [2]F to C");
        io::stdin()
            .read_line(&mut user_in)
            .expect("err io");

        let user_in = user_in.trim();

        if user_in == "1"{
            return 1;
        } else if user_in == "2"{
            return 2;
        } else {
            println!("\nPlease Type [1] or [2]");
            continue;
        }
    }
}


fn get_num() -> f32 {
    
    let mut user_in = String::new();

    loop {

        user_in.clear();

        println!("\nPlease Type your Temperature");

        io::stdin()
            .read_line(&mut user_in)
            .expect("err io");

        let temperature = match user_in.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return temperature;
    }
}


