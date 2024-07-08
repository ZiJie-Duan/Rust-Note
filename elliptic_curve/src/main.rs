
use std::{io::{self, Read, Write}, simd::f32x64};
use rand::Rng;

fn define_curve() -> (f64,f64){

    let a:f64 = loop {
        let mut user_in = String::new();

        print!("\nCurve_arg 'a' >");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut user_in)
            .expect("err define curve");

        match user_in.trim().parse() {
            Ok(num) => break num,
            Err(msg) => {println!("err: {}",msg); continue;},
        };
    };

    let b:f64 = loop {
        let mut user_in = String::new();

        print!("\nCurve_arg 'b' >");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut user_in)
            .expect("err define curve");

        match user_in.trim().parse() {
            Ok(num) => break num,
            Err(msg) => {println!("err: {}",msg); continue;},
        };
    };

    return (a,b);
}



fn main(){
    println!("hello");

    let (a,b) = define_curve();

    println!("a = {}, b = {}", a,b);
}