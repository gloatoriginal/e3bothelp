use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};

//save to user settings files
pub fn SaveUser(saveVector: &Vec<String>, whichFile: String) {  
    let mut file = File::create(format!("config/{}.txt", whichFile))
                            .expect("unable to create accounts file");
    for i in saveVector.iter(){
        writeln!(file, "{}", i.trim());
    }
}//save to user settings files

//read user settings files
//this is taking 3 different arguments so one for "accounts", "characters", and bots
pub fn AccountRead(whichFile: String) -> Vec<String> {
    let file = File::open(format!("config/{}.txt", whichFile))
            .expect("Unable to open Text file");
    let buff = BufReader::new(file);
    let mut returnVec = vec![];
    for i in buff.lines() {
        returnVec.push(i.unwrap());
    }
    returnVec
}//read user settings files