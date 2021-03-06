#![deny(missing_docs)]
//! HackAssembler
//! Converts Hack Assembly Code into Hack machine language. Hack is a computer specified in
//! "The elements of Computing Systems" (a.k.a. "nand2tetris") by Nisan and Schocken.


extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

pub use assembler::Assembler;

mod assembler;

fn main() {
    let matches = App::new("HackAssembler")
                          .version("0.7")
                          .author("thomasfermi")
                          .about("Converts Hack Assembly Code into Hack machine language. Hack is a computer specified in \"The elements of Computing Systems\" by Nisan and Schocken.")
                          .arg(Arg::with_name("assembly_input_file")
                               .help("Path to the Hack assembly file. File extension is asm.")
                               .required(true)
                               .index(1))         
                          .get_matches();

    let input_file_name : String = matches.value_of("assembly_input_file").unwrap().to_string();    

    let mut contents = String::new();

    // read input file
    { 
        let mut file = File::open(&input_file_name).expect("File not found.");
        file.read_to_string(&mut contents).expect("Could not read file");
    }

    let output_file_name = str::replace(&input_file_name,".asm", ".hack");

    let mut assembler = Assembler::new(&contents);
    let machine_language_program = assembler.assemble();

    // Write to output file
    {
        let path = Path::new(&output_file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                            display,
                            why.description()),
            Ok(file) => file,
        };

        match file.write_all(machine_language_program.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                why.description())
            },
            Ok(_) => println!("Successfully wrote machine code to {}", display),
        }
    }
}
