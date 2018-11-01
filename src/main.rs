#![deny(missing_docs)]
//! HackAssembler

extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;

mod parser;

use parser::Parser;

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(),2, "You have to give exactly one command line argument!");
    println!("args.len()={}",args.len());
    */


    let matches = App::new("HackAssembler")
                          .version("0.1")
                          .author("thomasfermi <mario.theers@gmail.com>")
                          .about("Converts Hack Assembly Code into Hack machine language. Hack is a computer specified \"The elements of Computing Systems\" by Nisan and Schocken.")
                          .arg(Arg::with_name("assembly_input_file")
                               .help("Path to the Hack assembly file. File extension is asm.")
                               .required(true)
                               .index(1))         
                          .get_matches();

    let input_file_name = matches.value_of("assembly_input_file").unwrap();

    let mut file = File::open(input_file_name).expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");

    let mut parser = Parser::new(&contents);
    parser.silly_print();
    println!("-----------------------------------");
    parser = Parser::new(&contents);
    parser.silly_print();



    println!("Hello, world!");
}
