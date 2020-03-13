use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use crate::usersettings::Users;

//save to user settings files
pub fn SaveUser(user: &Users) {  
    //write to accounts txt doc
    let mut file = File::create("config/accounts.txt")
        .expect("unable to create accounts file");
    for i in user.accounts.iter(){
        writeln!(file, "{}", i.trim())
                .expect("wasn't able to write to accounts file");
    }
    //write to characters txt doc
    file = File::create("config/characters.txt")
        .expect("unable to create characters file");
    for i in user.characters.iter(){
        writeln!(file, "{}", i.trim())
                .expect("wasn't able to write to accounts file");
    }
    file = File::create("config/bots.txt")
        .expect("unable to create bots file");
    for i in user.bots.iter(){
        writeln!(file, "{}", i.trim())
                .expect("wasn't able to write to accounts file");
    }
    
}//save to user settings files

//read user settings files
pub fn UserRead(userFileType: String) -> Vec<String>{
    let file = File::open("config/".to_owned() + &userFileType + &".txt".to_owned())
            .expect("Unable to open Text file");
    let buff = BufReader::new(file);
    let mut returnVec = vec![];
    for i in buff.lines() {
        returnVec.push(i.unwrap());
    }
    returnVec
}//read user settings files