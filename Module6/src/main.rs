/*
    Author: FM (fm@s0.nz)

    TODO:
    - Remove any useless code.
    - Refactor the structure of the code for the design to make sense.
    - Find a way to automagically figure out the name of the input file. (Read the basename.)

*/

use std::ops::Deref;
use std::ptr::eq;
use std::{collections::HashMap, vec};
use std::{env, fs};
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use chrono::prelude::*;



#[derive(Hash, Eq, PartialEq, Debug)]
struct BinaryMap {
    symbol: String,
    binary: String
}


impl BinaryMap {
    fn new(symbol: &str, binary: &str) -> BinaryMap {
        BinaryMap { symbol: symbol.to_string(), binary: binary.to_string() }
    }
}


struct a_mapping { 
    value: i16
}

impl a_mapping {
    fn convert_to_bin(&self) -> String {
        let mut binary_string = String::new();
        let mut num = self.value;
        if num  == 0 {
            binary_string.insert(0, '0');
        }
        while num > 0 {
            let remainder = num % 2;
            binary_string.insert(0, char::from_digit(remainder.try_into().unwrap(), 10).unwrap());
            num /= 2;
        };

        while binary_string.len() != 15 {
            binary_string.insert(0, '0');
        }
        binary_string
    }
}



struct c_mapping {
    dest: Vec<BinaryMap>,
    comp: Vec<BinaryMap>,
    jump: Vec<BinaryMap>
}


impl c_mapping {
    fn find(&self, map_type: &str, entry: &str) -> String {
        if map_type == "dest" {
            for types in &self.dest {
                if entry == types.symbol {
                    return types.binary.to_string()
                }
            }
        }


        if map_type == "comp" {
            for types in &self.comp {
                if entry == types.symbol {
                    return types.binary.to_string()
                }
            }
        }


        if map_type == "jump" {
            for types in &self.jump {
                if entry == types.symbol {
                    return types.binary.to_string()
                }
            }
        }             

        return "0".to_string()

    }
    fn new() -> c_mapping {
        let dest_stat: Vec<BinaryMap> = vec![
            BinaryMap::new("null", "000"), 
            BinaryMap::new("M", "001"),
            BinaryMap::new("D", "010"),    
            BinaryMap::new("MD", "011"),      
            BinaryMap::new("A", "100"),            
            BinaryMap::new("AM", "101"),            
            BinaryMap::new("AD", "110"),            
            BinaryMap::new("AMD", "111"),           
            ];

        let jump_stat: Vec<BinaryMap> = vec![
            BinaryMap::new("null", "000"),
            BinaryMap::new("JGT", "001"),
            BinaryMap::new("JEQ", "010"),
            BinaryMap::new("JGE", "011"),
            BinaryMap::new("JLT", "100"),
            BinaryMap::new("JNE", "101"),
            BinaryMap::new("JLE", "110"),
            BinaryMap::new("JMP", "111")
        ];

        let comp_stat: Vec<BinaryMap> = vec![
            BinaryMap::new("0", "0101010"),
            BinaryMap::new("1", "0111111"),
            BinaryMap::new("-1", "0111010"),
            BinaryMap::new("D", "0001100"),
            BinaryMap::new("A", "0110000"),
            BinaryMap::new("!D", "0001101"),
            BinaryMap::new("!A", "0110001"),
            BinaryMap::new("-D", "0001111"),
            BinaryMap::new("D+1", "0011111"),
            BinaryMap::new("A+1", "0110111"),
            BinaryMap::new("D-1", "0001110"),
            BinaryMap::new("A-1", "0110010"),
            BinaryMap::new("D+A", "0000010"),
            BinaryMap::new("D-A", "0010011"),
            BinaryMap::new("A-D", "0000111"),
            BinaryMap::new("D&A", "0000000"),
            BinaryMap::new("D|A", "0010101"), 
            // A = 0 ^
            // A = 1 now
            BinaryMap::new("M", "1110000"),
            BinaryMap::new("!M", "1110001"),
            BinaryMap::new("-M", "1110011"),
            BinaryMap::new("M+1", "1110111"),
            BinaryMap::new("M-1", "1110010"),
            BinaryMap::new("D+M", "1000010"),
            BinaryMap::new("D-M", "1010011"),
            BinaryMap::new("M-D", "1000111"),
            BinaryMap::new("D&M", "1000000"),
            BinaryMap::new("D|M", "1010101")
         ];
        c_mapping {
            dest: dest_stat, jump: jump_stat, comp: comp_stat
        }
    }
}


#[derive(Hash, Eq, PartialEq, Debug)]
struct SymbolTableEntry {
    symbol: String,
    value: i32
}

impl SymbolTableEntry {
    fn new(symbol: &str, value: i32) -> Self {
        SymbolTableEntry { symbol: symbol.to_string(), value: value }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct SymbolTable {
    data: Vec<SymbolTableEntry>,
    counter: i16
}


impl SymbolTable {
    fn find_value(&self, key: &str) -> i32 {
        for entry in &self.data {
            if key.to_string() == entry.symbol {
               return entry.value.try_into().unwrap()
            }
        }

        return 300000
    }

    fn add(&mut self, entry: SymbolTableEntry, variable: bool) -> () {

        for exist in &self.data {
            if exist.symbol  == entry.symbol {
                println!("Symbol {} already exists.", entry.symbol);
                return;
            }
        }


        self.data.push(entry);
        if variable {
            self.counter = self.counter + 1;
        }        
    }
    fn new() -> Self {
        let init_data: Vec<SymbolTableEntry> = vec![
            SymbolTableEntry::new("R15", 15),
            SymbolTableEntry::new("R14", 14),
            SymbolTableEntry::new("R13", 13),
            SymbolTableEntry::new("R12", 12),
            SymbolTableEntry::new("R11", 11),
            SymbolTableEntry::new("R10", 10),
            SymbolTableEntry::new("R9", 9),
            SymbolTableEntry::new("R8", 8),
            SymbolTableEntry::new("R7", 7),
            SymbolTableEntry::new("R6", 6),
            SymbolTableEntry::new("R5", 5),
            SymbolTableEntry::new("R4", 4),
            SymbolTableEntry::new("R3", 3),
            SymbolTableEntry::new("R2", 2),
            SymbolTableEntry::new("R1", 1),
            SymbolTableEntry::new("R0", 0),

            SymbolTableEntry::new("SCREEN", 16384),
            SymbolTableEntry::new("KBD", 24576),
            SymbolTableEntry::new("SP", 0),
            SymbolTableEntry::new("LCL", 1),
            SymbolTableEntry::new("ARG", 2),
            SymbolTableEntry::new("THIS", 3),
            SymbolTableEntry::new("THAT", 4)

        ];
        SymbolTable { data: init_data, counter: 16 }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Parser {
    file: String,
    data: Vec<String>
}

impl Parser {

    fn read_lines(&mut self) -> ()  {
        let path: &Path = Path::new(&self.file);
        let file: Result<File, io::Error> = File::open(path);

        let lines = io::BufReader::new(
    match file {
            Ok(t) => t,
            Err(e) => panic!("File not found! Info: {}", e)
           }
        ).lines();

        for line in lines.map_while(Result::ok) {
            self.data.push(line);
        } 

    }

    fn new( file: &str) -> Self {
        Parser { file: file.to_string(), data: Vec::new() }
    }
}

#[derive(Debug)]
struct BinaryCode {
    code: Vec<String>
}

impl BinaryCode {
    fn add(&mut self, line: &str) -> () {
        self.code.push(line.to_string());
    }

    fn new() -> Self {
        BinaryCode { code: Vec::new() }
    }
}


fn first_pass(data: &Vec<String>, symbols: &mut SymbolTable) {
    let mut counter: i16 = 0;

    for line in data {
        let trimmed_line = line.trim().to_string();
        
        if (trimmed_line.starts_with("/") | trimmed_line.is_empty()) == true {
            continue;
        }

        if trimmed_line.starts_with("(") == true {
            let length: usize = trimmed_line.len() - 1;
            let symbol_name = &trimmed_line[1..length].to_string();
            symbols.add(SymbolTableEntry::new(&symbol_name, (counter).into()), false);
            continue;
        }

        counter = counter + 1;

    }


}
fn second_pass(data: &Vec<String>, symbols: &mut SymbolTable, binary: &mut BinaryCode) {
    let mut counter: i16 = 0;
    let c_mapping = c_mapping::new();

    for line in data {
        let trimmed_line = line.trim().to_string();
        
        if (trimmed_line.starts_with("/") | trimmed_line.is_empty()) == true {
            continue;
        }

        if trimmed_line.starts_with("(") {
            continue; 
        }
        if trimmed_line.starts_with("@") {
            let mut value: i16 = 0;
            if trimmed_line.to_ascii_lowercase().chars().any(|c| matches!(c, 'a'..='z')) {
                let symbol_value = symbols.find_value(&trimmed_line[1..]);
                if symbol_value == 300000 {
                    value = symbols.counter;
                    symbols.add(SymbolTableEntry::new(&trimmed_line[1..], symbols.counter.try_into().unwrap()), true);
                } else {
                    value = symbol_value.try_into().unwrap();
                }
            } else {
                value = i16::from_str_radix(&trimmed_line[1..], 10).unwrap().try_into().unwrap();
            }

            let mut output = a_mapping{value: value}.convert_to_bin();
            output.insert(0, '0');
            binary.add(&output);
            continue;
        }

        let mut dest_jmp = String::new();
        let mut comp_jmp = String::new();
        let mut jmp_jmp = String::new();

        /* Assuming none of the other match, we're talking about a C-instruction then. */
        let equal_check =  match trimmed_line.find('=') {
            Some(t) => t,
            None => 0,
            _ => 0
        };

        let semicolon_check =  match trimmed_line.find(';') {
            Some(t) => t,
            None => 0,
            _ => 0
        };

        if equal_check == 0 {
            dest_jmp = c_mapping.find("dest", "null");
            comp_jmp = c_mapping.find("comp",  &trimmed_line[0..semicolon_check]);
            jmp_jmp = c_mapping.find("jump", &trimmed_line[semicolon_check+1..]);
            



        } else if semicolon_check == 0 {
            dest_jmp = c_mapping.find("dest", &trimmed_line[0..equal_check]);
            comp_jmp = c_mapping.find("comp",  &trimmed_line[equal_check+1..]);
            jmp_jmp = c_mapping.find("jump", "null");            

        } else {
            dest_jmp = c_mapping.find("dest", &trimmed_line[0..equal_check]);
            comp_jmp = c_mapping.find("comp",  &trimmed_line[equal_check+1..semicolon_check]);
            jmp_jmp = c_mapping.find("jump", &trimmed_line[semicolon_check+1..]);
        }


        let c_instruction = format!("111{}{}{}", comp_jmp,dest_jmp,jmp_jmp);

        binary.add(&c_instruction);


        counter = counter + 1;

    }

}

fn output(binary: &mut BinaryCode, name: String){ 
    let path = "./output";
    fs::create_dir_all(&path).unwrap();
    //println!("{}", format!("{}/{}.hack", &path, &name))  ;


    fs::write(format!("{}/{}.hack", &path, name),binary.code.join("\n")).expect("Unable to write file!");

}

fn main() {
    /* Initalise the SymbolTable */
    let mut symboltable = SymbolTable::new();

    /* Read the arguments */
    let mut argv: env::Args= env::args();
    let arg1: Option<String> = argv.nth(1);
    match &arg1.as_deref() {
        Some(s) => Some(s),
        None => panic!("No argument provided!")
    };

    let copy = arg1.clone().unwrap();
    let slash_find = &copy.find('/').unwrap();

    // let mut filename= match copy.find('.') {
    //     Some(s) => &copy[slash_find..s],
    //     None => "Unknown"
    // };



    let date_as_string = Utc::now().to_string();
    /* Create new object. */
    let file: String  = arg1.unwrap();
    let mut parser = Parser::new(&file);
    parser.read_lines();

    first_pass(&parser.data, &mut symboltable);

    /* Create assembly layer */
    let mut assemblycode: BinaryCode = BinaryCode::new();
    second_pass(&parser.data, &mut symboltable, &mut assemblycode);

    output(&mut assemblycode, format!("{}-compiled",date_as_string));

}
