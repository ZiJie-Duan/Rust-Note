fn say_something(text: &str) {
    println!("Say: {}", text);
}

fn main() {
    let sentence = "hello how are you, what are you doing.";
    for word in sentence.split(" ") {
        say_something(word);
    }

    let sentence = format!("my name is {}", "zijie");
    say_something(&sentence);
}
