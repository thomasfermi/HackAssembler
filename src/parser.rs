
use std::str::Lines;
use std::collections::HashMap;

//TODO: enforce documentation; try clippy; when parsing is not possible return error with line number
//TODO: Some doc unit tests

#[derive(Debug)]
pub enum HackMemory {
    A,
    D,
    M {address : usize}
}

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
    L {line : usize},
    InvalidCommand,
}



pub struct Parser<'a> {
    input_iterator : Lines<'a>,
    current_command_string : Option<String>,
    compute_dictionary : HashMap<String,String>,
    dest_dictionary : HashMap<String,String>,
    jump_dictionary : HashMap<String,String>,
}

impl<'a>  Parser<'a> {
    pub fn new(input : &'a String ) -> Self {
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
            input_iterator : input.lines(),
            current_command_string : None,
            compute_dictionary,
            dest_dictionary,
            jump_dictionary
        }
    }

    pub fn get_machine_language_command(&self, command : Command) -> String  {
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
            }
            _ => "".to_string(),
        }
    }

    pub fn advance(&mut self) {
        if let Some(line) = self.input_iterator.next(){
            // copy string slice to a string
            let mut s : String = line.to_string();
            // remove whitespace and comments
            s = str::replace(&s, " ", "");
            let comment_offset = s.find("//").unwrap_or(s.len());
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

    pub fn get_command(&self) -> Command {
        // TODO: more thorough checking for invalid commands
        let c = self.current_command_string.as_ref().unwrap();
        let find_at = c.find("@");
        let find_equal = c.find("=");
        let find_semicolon = c.find(";");
        let find_paranthesis_open = c.find("(");
        let find_paranthesis_closed = c.find(")");

        if find_at!= None && find_equal == None && find_semicolon == None {
            return Command::A {address : Self::parse_a_command(c)};
        }
        else if find_at == None && (find_equal!=None || find_semicolon != None) {
            return Command::C {command : self.parse_c_command(c)};
        }
        else if find_paranthesis_open != None && find_paranthesis_closed != None {
            return Command::L {line : Self::parse_l_command(c)};
        }
        else {
            return Command::InvalidCommand; //TODO: This is an error and should be handled!
        }
    }

    pub fn assemble(&mut self) -> String{
        let mut output  = String::new();
        self.advance();
        while self.current_command_string != None {          
            output += &format!("{}\n", self.get_machine_language_command(self.get_command()));  
            self.advance();
        }
        return output;
    }

    pub fn silly_print(&mut self){
        self.advance();
        while self.current_command_string != None {          
            //println!("{}", self.current_command_string.as_ref().unwrap());  
            //println!("{:?}", self.get_command());  
            println!("{}", self.get_machine_language_command(self.get_command()));     
            self.advance();
        }
    }

    /// ```
    /// let a_code : String = "@123";
    /// assert_eq!(parse_a_command(&a_code), 123)
    /// ```
    fn parse_a_command(a_code : &String) -> usize { //TODO: return error in case that parsing doesn't work
            let mut c = a_code.to_string();
            let find_at = c.find("@").unwrap()+1;
            let command : String = c.drain(find_at..).collect();
            //println!("command={}", command);
            return command.parse::<usize>().unwrap();
    }

    fn parse_c_command(&self, c_code : &String) -> CCommand {
        let mut c = c_code.to_string();
        let find_equal = c.find("=");

        let dest : String;
        let comp : String;
        let jmp : String;

        //println!("Ccode={}",Ccode);

        if find_equal != None {
            dest =  c.drain(..find_equal.unwrap()).collect();
            let find_semicolon = c.find(";").unwrap_or(c.len());
            comp =  c.drain(1..find_semicolon).collect();
            jmp  =  c.drain(1..).collect();
        }
        else {
            dest = "".to_string();
            let find_semicolon = c.find(";").unwrap_or(c.len());
            comp =  c.drain(..find_semicolon).collect();
            jmp  =  c.drain(1..).collect();
        }
        //println!("Done");

        return CCommand {dest, comp,jmp};
    }


    fn parse_l_command(l_code : &String) -> usize { //TODO: return error in case that parsing doesn't work
            let mut c = l_code.to_string();
            let find_paranthesis_open = c.find("(").unwrap()+1;
            let find_paranthesis_closed = c.find(")").unwrap();
            let command : String =  c.drain(find_paranthesis_open..find_paranthesis_closed).collect();
            return command.parse::<usize>().unwrap();
    }
    
}



