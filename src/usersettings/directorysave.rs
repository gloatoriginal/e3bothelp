use std::fs::{File};
use std::io::prelude::*;
use std::io::{Read, Write};


pub fn SaveDirectory(directory: &String, whichFile: String) {
    let mut file = File::create(format!("config/{}.txt", whichFile))
            .expect("unable to create txt file in save directory");
    writeln!(file, "{}", directory.trim()); 
}

pub fn ReadDirectory(whichFile: String) -> String {
    let mut file = File::open(format!("config/{}.txt", whichFile))
                        .expect("Unable to open Text file");
    let mut returnString = String::new();
    file.read_to_string(&mut returnString).expect("unable to open");
    println!("{}", returnString);
    returnString
}