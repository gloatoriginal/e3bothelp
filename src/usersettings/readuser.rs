use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;

use crate::usersettings::Users;

pub fn ReadUser() -> Users {
    let user = Users {
        accounts: accountsRead(),
        characters: charactersRead(),
        bots: botsRead(),
    };
    user
}

fn accountsRead() -> Vec<String>{
    let file = File::open("config/accounts.txt")
            .expect("unable to open accounts.txt");
    let buff = BufReader::new(file);
    let mut returnVec = vec![];
    for i in buff.lines() {
        returnVec.push(i.unwrap() + "\n");
    }
    returnVec
}

fn charactersRead() -> Vec<String> {
    let file = File::open("config/characters.txt")
            .expect("unable to open characters.txt");
    let buff = BufReader::new(file);
    let mut returnVec = vec![];
    for i in buff.lines() {
        returnVec.push(i.unwrap() + "\n");
    }
    returnVec
}

fn botsRead() -> Vec<bool> {
    let file = File::open("config/bots.txt")
            .expect("unable to open characters.txt");
    let buff = BufReader::new(file);
    let mut returnVec = vec![];
    for i in buff.lines() {
        if i.unwrap().trim() == "true" { returnVec.push(true) }
        else { returnVec.push(false) }
    }
    returnVec
}
