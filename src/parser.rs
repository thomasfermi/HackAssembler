
use std::str::Lines;

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

    pub fn silly_print(&mut self){
        self.advance();
        while self.current_command != None {          
            println!("{}", self.current_command.as_ref().unwrap());            
            self.advance();
        }
    }

    
}