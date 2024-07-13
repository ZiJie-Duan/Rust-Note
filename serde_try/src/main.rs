use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct HeartBeat {
    id : String
}

#[derive(Serialize, Deserialize, Debug)]
enum CommandType {
    SYSTEM,
    USER
}

#[derive(Serialize, Deserialize, Debug)]
struct CommandPack {
    ctype : CommandType,
    head : String,
    args : Vec<String>
}


fn main() {
    let cp = CommandPack{
        ctype : CommandType::SYSTEM,
        head : String::from("ls"),
        args : vec!["abc".into(), "edf".into()]
    };

    let serialized = serde_json::to_string(&cp).unwrap();
    println!("serialized = {}", serialized);

    let v = Vec::from(serialized);

    let s = String::from_utf8_lossy(&v);


    let deserialized: CommandPack = serde_json::from_str(&s).unwrap();
    println!("deserialized = {:?}", deserialized);

    println!("this {}", deserialized.args[1]);
}