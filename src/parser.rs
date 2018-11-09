//! Parser

extern crate regex;
use self::regex::Regex;
use std::str::Lines;
use std::collections::HashMap;



#[derive(Debug)]
pub struct CCommand {
    dest : String,
    comp : String,
    jmp : String,
}


#[derive(Debug)]
pub enum Command {
    A {address:usize},
    C {command : CCommand},
}

/// Parser
pub struct Parser<'a> {
    input_string : &'a str,
    input_iterator : Lines<'a>,
    current_command_string : Option<String>,
    current_line_number : usize,
    running_symbol_memory_address : usize,
    symbol_table : HashMap<String, usize>,
    compute_dictionary : HashMap<String,String>,
    dest_dictionary : HashMap<String,String>,
    jump_dictionary : HashMap<String,String>,
}

impl<'a>  Parser<'a> {
    /// Creates new Parser
    pub fn new(input : &'a str) -> Self {

        let mut symbol_table = HashMap::new();
        symbol_table.insert("SP".to_string(), 0);
        symbol_table.insert("LCL".to_string(), 1);
        symbol_table.insert("ARG".to_string(), 2);
        symbol_table.insert("THIS".to_string(), 3);
        symbol_table.insert("THAT".to_string(), 4);
        symbol_table.insert("R0".to_string(), 0);        
        symbol_table.insert("R1".to_string(), 1);
        symbol_table.insert("R2".to_string(), 2);
        symbol_table.insert("R3".to_string(), 3);
        symbol_table.insert("R4".to_string(), 4);
        symbol_table.insert("R5".to_string(), 5);
        symbol_table.insert("R6".to_string(), 6);
        symbol_table.insert("R7".to_string(), 7);
        symbol_table.insert("R8".to_string(), 8);
        symbol_table.insert("R9".to_string(), 9);
        symbol_table.insert("R10".to_string(), 10);
        symbol_table.insert("R11".to_string(), 11);
        symbol_table.insert("R12".to_string(), 12);
        symbol_table.insert("R13".to_string(), 13);
        symbol_table.insert("R14".to_string(), 14);
        symbol_table.insert("R15".to_string(), 15);
        symbol_table.insert("SCREEN".to_string(), 16384);
        symbol_table.insert("KBD".to_string(), 24576);


        let mut compute_dictionary = HashMap::new();
        compute_dictionary.insert("0".to_string(),   "0101010".to_string());
        compute_dictionary.insert("1".to_string(),   "0111111".to_string());
        compute_dictionary.insert("-1".to_string(),  "0111010".to_string());
        compute_dictionary.insert("D".to_string(),   "0001100".to_string());
        compute_dictionary.insert("A".to_string(),   "0110000".to_string());
        compute_dictionary.insert("M".to_string(),   "1110000".to_string());
        compute_dictionary.insert("!D".to_string(),  "0001101".to_string());
        compute_dictionary.insert("!A".to_string(),  "0110001".to_string());
        compute_dictionary.insert("!M".to_string(),  "1110001".to_string());
        compute_dictionary.insert("-D".to_string(),  "0001111".to_string());
        compute_dictionary.insert("-A".to_string(),  "0110011".to_string());
        compute_dictionary.insert("-M".to_string(),  "1110011".to_string());
        compute_dictionary.insert("D+1".to_string(), "0011111".to_string());
        compute_dictionary.insert("A+1".to_string(), "0110111".to_string());
        compute_dictionary.insert("M+1".to_string(), "1110111".to_string());
        compute_dictionary.insert("D-1".to_string(), "0001110".to_string());
        compute_dictionary.insert("A-1".to_string(), "0110010".to_string());
        compute_dictionary.insert("M-1".to_string(), "1110010".to_string());
        compute_dictionary.insert("D+A".to_string(), "0000010".to_string());
        compute_dictionary.insert("D+M".to_string(), "1000010".to_string());
        compute_dictionary.insert("D-A".to_string(), "0010011".to_string());
        compute_dictionary.insert("D-M".to_string(), "1010011".to_string());
        compute_dictionary.insert("A-D".to_string(), "0000111".to_string());
        compute_dictionary.insert("M-D".to_string(), "1000111".to_string());
        compute_dictionary.insert("D&A".to_string(), "0000000".to_string());
        compute_dictionary.insert("D&M".to_string(), "1000000".to_string());
        compute_dictionary.insert("D|A".to_string(), "0010101".to_string());
        compute_dictionary.insert("D|M".to_string(), "1010101".to_string());

        let mut dest_dictionary = HashMap::new();
        dest_dictionary.insert("null".to_string(), "000".to_string());
        dest_dictionary.insert("".to_string(),     "000".to_string());
        dest_dictionary.insert("M".to_string(),    "001".to_string());
        dest_dictionary.insert("D".to_string(),    "010".to_string());
        dest_dictionary.insert("MD".to_string(),   "011".to_string());
        dest_dictionary.insert("A".to_string(),    "100".to_string());
        dest_dictionary.insert("AM".to_string(),   "101".to_string());
        dest_dictionary.insert("AD".to_string(),   "110".to_string());
        dest_dictionary.insert("AMD".to_string(),  "111".to_string());

        let mut jump_dictionary = HashMap::new();
        jump_dictionary.insert("null".to_string(), "000".to_string());
        jump_dictionary.insert("".to_string(),     "000".to_string());
        jump_dictionary.insert("JGT".to_string(),  "001".to_string());
        jump_dictionary.insert("JEQ".to_string(),  "010".to_string());
        jump_dictionary.insert("JGE".to_string(),  "011".to_string());
        jump_dictionary.insert("JLT".to_string(),  "100".to_string());
        jump_dictionary.insert("JNE".to_string(),  "101".to_string());
        jump_dictionary.insert("JLE".to_string(),  "110".to_string());
        jump_dictionary.insert("JMP".to_string(),  "111".to_string());

        Parser {
            input_string : input,
            input_iterator : input.lines(),
            current_command_string : None,
            current_line_number : 0,
            running_symbol_memory_address : 16,
            symbol_table,
            compute_dictionary,
            dest_dictionary,
            jump_dictionary
        }
    }

    fn reset_input_iterator(&mut self)
    {
        self.current_line_number=0;
        self.input_iterator = self.input_string.lines();
    }

    fn get_machine_language_command(&self, command : Command) -> String  {
        match command {
            Command::A {address} => {
                let s = format!("{:b}", address); // convert to binary
                format!("{:0>16}", s) // append zeros to the left so that it is a string of size 16
            },
            Command::C {command} => {
                format!("111{comp}{dest}{jump}",
                    comp = self.compute_dictionary[&command.comp],
                    dest = self.dest_dictionary[&command.dest],
                    jump = self.jump_dictionary[&command.jmp]
                )
            },
        }
    }

    fn advance(&mut self) {
        if let Some(line) = self.input_iterator.next(){
            self.current_line_number += 1;
            // copy string slice to a string
            let mut s : String = line.to_string();
            // remove whitespace and comments
            s = str::replace(&s, " ", "");
            let comment_offset = s.find("//").unwrap_or_else(|| s.len());
            let command_string : String = s.drain(..comment_offset).collect();
            // if there is a valid command, store it in self.current_command_string. Otherwise, advance further
            if command_string.is_empty(){
                self.advance();
            }
            else {
                self.current_command_string = Some(command_string);
            }            
        }
        else {
            self.current_command_string = None;
        }
    }

    fn get_l_symbol(&mut self) -> Option<String> {
        let c = self.current_command_string.as_ref().unwrap();
        lazy_static! {
            static ref re_l_command : Regex = Regex::new(r"^\(([_0-9a-zA-Z\.\$:]+)\)").unwrap();
        }
        if re_l_command.is_match(c) {
            let caps = re_l_command.captures(c).unwrap();
            let symbol_name : String = caps.get(1).map_or("", |m| m.as_str()).to_string(); 
            Some(symbol_name)
        }
        else {
            None
        }
    }

    fn get_command(&mut self) -> Option<Command> {
        // TODO: more thorough checking for invalid commands
        let c = self.current_command_string.as_ref().unwrap();
        lazy_static! {
            static ref re_l_command : Regex = Regex::new(r"^\(([_0-9a-zA-Z\.\$:]+)\)").unwrap();
            static ref re_a_command : Regex = Regex::new(r"^@([_0-9a-zA-Z\.\$:]+)").unwrap();
            static ref re_c_command : Regex = Regex::new(r"^([ADM]*)(=?)([-\+01DAM!&\|]+)(;?)([JGTEQNLMP]*)").unwrap();
        }

        if re_c_command.is_match(c) {
            let caps = re_c_command.captures(c).unwrap();
            let dest = caps.get(1).map_or("", |m| m.as_str());
            let comp = caps.get(3).map_or("", |m| m.as_str());
            let jmp = caps.get(5).map_or("", |m| m.as_str());     
            Some(Command::C {command: CCommand{dest: dest.to_string(), comp: comp.to_string(), jmp: jmp.to_string()}})     
        }
        else if re_a_command.is_match(c) {
            let caps = re_a_command.captures(c).unwrap();
            let address_or_symbol : String = caps.get(1).map_or("", |m| m.as_str()).to_string(); 

            let address_number = match address_or_symbol.parse::<usize>() {
                Ok(number) => number,
                _ => {
                    if !self.symbol_table.contains_key(&address_or_symbol) {
                        self.symbol_table.insert(address_or_symbol.clone(), self.running_symbol_memory_address);
                        self.running_symbol_memory_address += 1;
                    }

                    self.symbol_table[&address_or_symbol]
                },
            };
            Some(Command::A {address : address_number})
        }
        else if re_l_command.is_match(c) {
            None
        }
        else {
            println!("Assembler failed. Syntax error at line {} of the input file.", self.current_line_number);
            ::std::process::exit(1);
        }
    }

    /// Converts the input file to a Hack machine language program. The 0's and 1's in the machine language program are written to a String.
    pub fn assemble(&mut self) -> String{
        self.build_symbol_table();
        self.reset_input_iterator();
        
        let mut output  = String::new();
        self.advance();
        while self.current_command_string != None {        
            if let Some(command) = self.get_command(){
                output += &format!("{}\n", self.get_machine_language_command(command));
            }    
            self.advance();
        }
        output
    }


    fn build_symbol_table(&mut self) {
        let mut line_counter : usize = 0;
        self.advance();
        while self.current_command_string != None {
            if let Some(symbol_name) = self.get_l_symbol(){
                self.symbol_table.insert(symbol_name, line_counter);
            }
            else {
                line_counter +=1;
            }
            self.advance();
        }
    }
    
}



