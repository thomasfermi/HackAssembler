
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
            self.current_command = Some(line.into());
        }
        else {
            self.current_command = None;
        }
    }

    pub fn silly_print(&mut self){
        self.advance();
        while self.current_command != None {
            println!("{}", self.get_current_command_without_comments_and_whitespace());
            self.advance();
        }
    }

    pub fn get_current_command_without_comments_and_whitespace(&self) -> String {
        let mut s = self.current_command.clone().unwrap_or(String::new());
        s = str::replace(&s, " ", "");

        let comment_offset = s.find("//").unwrap_or(s.len());
        let ret : String = s.drain(..comment_offset).collect();        
        return ret;
    }

    
}