use core::include;
use core::option::Option;
use core::prelude::v1::derive;
use core::str::Lines;
use std::fs::exists;
use std::{env, fs};

#[derive(Debug)]
enum FunctionKind {
    Arithmetic,
    Memory
}

#[derive(Debug)]
struct Command {
    kind: FunctionKind,
    function: String,
    arguments: Vec<String>,
}

impl Command {
    fn math(cmd: String) -> Self {
        Self {
            kind: FunctionKind::Arithmetic,
            function: cmd,
            arguments: [].to_vec(),
        }
    }

    fn memory(cmd: String, args: Vec<String>) -> Self {
        Self{
            kind: FunctionKind::Memory,
            function: cmd,
            arguments: args,
        }
    }
}

#[derive(Debug)]
struct Parser {
    commands: Vec<Command>
}

impl Parser {
    fn new() -> Parser {
        Parser { commands: Vec::new(), }
    }

    fn add_cmds(&mut self, mut iterator: Lines) {
        for line in iterator.by_ref() {
            if line.starts_with("//") || line.chars().count() == 0 {
                continue;
            }

            println!("{}", line);
            let split_string = line.split(" ").collect::<Vec<_>>();
            let parameters: Vec<String> = split_string.clone().split_off(1).iter().map(|&e| String::from(e)).collect();
            match split_string[0] { // needs to be extended once we get other types of commands
                                    // going.
                "push"|"pop" => self.commands.push(Command::memory((&split_string[0]).to_string(), parameters)),
                &_ => self.commands.push(Command::math((&split_string[0]).to_string())),
            }

                
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
 
    let option = Options::new(&args);

    if fs::exists(&option.file_path).expect("Unable to read file.") {
        println!("{}", &args[1])
    } else {
        panic!("file non existant");
    };

    let strings = fs::read_to_string(&option.file_path).expect("Unable to turn it into strings.");
    
    let mut parser = Parser::new();

    parser.add_cmds(strings.lines());

    dbg!(&parser);
}


struct Options {
    file_path: String,
}

impl Options {
    fn new(args: &[String]) -> Options {
        if args.len() < 2 {
            panic!("missing file argument");
        }
        Options { file_path: args[1].clone(), }
    }
}
