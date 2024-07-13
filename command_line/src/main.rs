
enum Command {
    
}

enum Options {
    
}

struct CommandLine {
    command : Command,
    options : Options,
    arguments : Vec<Arguments>,
}


struct CommandLineStr {
    command : String,
    options : Vec<String>,
    arguments : Vec<String>,
}

impl CommandLineStr {
    fn new() -> CommandLineStr {
        CommandLineStr {
            command : String::new(),
            options : Vec::new(),
            arguments : Vec::new(),
        }
    }

    fn to_command_line(&self) -> CommandLine {
        CommandLine {
            command : Command::new(self.command),
            options : Options::new(self.options),
            arguments : Arguments::new(self.arguments),
        }
    }
}


fn get_a_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}


fn main() {
    println!("Hello, world!");
}





















