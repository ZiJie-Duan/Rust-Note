#[derive(Debug)]
enum Gender {
    Man,
    Woman,
    Batman,
    Other,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Other
    }
}

#[derive(Default, Debug)]
struct Person {
    name: String,
    age: i8,
    gender: Gender,
}

fn main() {
    println!("Hello, world!");
    let p = Person::default();
    println!("{:?}", p);
    let p2 = Person {
        name: "hello".to_string(),
        ..Default::default()
    };
    println!("{:?}", p2);
}
