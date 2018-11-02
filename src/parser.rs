
use std::str::Lines;

#[derive(Debug)]
pub enum CommandType {
    //TODO: attach data to different types, such as address for A, (dest,comp,jmp) for C and symbol for L
    A_command,
    C_command,
    L_command,
    invalid_command,
}

pub struct Parser<'a> {
    input_iterator : Lines<'a>,
    current_command : Option<String>,
}

impl<'a>  Parser<'a> {
    pub fn new(input : &'a String ) -> Self {
        Parser {input_iterator : input.lines(), current_command : None}
    }

    pub fn advance(&mut self) {
        if let Some(line) = self.input_iterator.next(){
            // copy string slice to a string
            let mut s : String = line.to_string();
            // remove whitespace and comments
            s = str::replace(&s, " ", "");
            let comment_offset = s.find("//").unwrap_or(s.len());
            let command : String = s.drain(..comment_offset).collect();
            // if there is a valid command, store it in self.current_command. Otherwise, advance further
            if command.is_empty(){
                self.advance();
            }
            else {
                self.current_command = Some(command);
            }            
        }
        else {
            self.current_command = None;
        }
    }

    pub fn command_type(&self) -> CommandType {
        // TODO: more thorough checking for invalid commands
        let c = self.current_command.as_ref().unwrap();
        let find_at = c.find("@");
        let find_equal = c.find("=");
        let find_semicolon = c.find(";");
        let find_paranthesis_open = c.find("(");
        let find_paranthesis_closed = c.find(")");

        if find_at!= None && find_equal == None && find_semicolon == None {
            return CommandType::A_command;
        }
        else if find_at == None && (find_equal!=None || find_semicolon != None) {
            return CommandType::C_command;
        }
        else if find_paranthesis_open != None && find_paranthesis_closed != None {
            return CommandType::L_command;
        }
        else {
            return CommandType::invalid_command; //TODO: This is an error and should be handled!
        }

    }

    pub fn silly_print(&mut self){
        self.advance();
        while self.current_command != None {          
            println!("{}", self.current_command.as_ref().unwrap());  
            println!("{:?}", self.command_type());          
            self.advance();
        }
    }

    
}