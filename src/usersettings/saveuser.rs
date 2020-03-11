use std::fs::File;
use std::io::prelude::*;
use crate::usersettings::Users;

pub fn SaveUser(user: &Users) {
    //write to accounts txt doc
    let mut file = File::create("config/accounts.txt")
        .expect("unable to create accounts file");
    for i in user.accounts.iter(){
        file.write(i.as_bytes())
            .expect("unable to write to accounts file");
    }
    //write to characters txt doc
    file = File::create("config/characters.txt")
        .expect("unable to create characters file");
    for i in user.characters.iter(){
        file.write(i.as_bytes())
            .expect("unable to write to characters file");
    }
    file = File::create("config/bots.txt")
        .expect("unable to create bots file");
    for i in user.bots.iter(){
        if *i {
            file.write("true\n".as_bytes())
                .expect("unable to write to bots file");
        } else {
            file.write("false\n".as_bytes())
                .expect("unable to write to bots file");
        }
    }
}
