use std::fs::File;
use std::io::prelude::*;


pub fn UserCreate(GetInput: &dyn Fn() -> String, mut user: Users) -> Users{
    //create mutable strings to assign for user settings struct
    println!("Please enter your Account name");
    user.accounts.push(GetInput());
    println!("Please enter your Character name");
    user.characters.push(GetInput());
    println!("Is this going to be a Bot Character? y or n default is y.");
    let botCheck = GetInput();
    if botCheck.trim() == "n" { user.bots.push(false); }
    else { user.bots.push(true); }
    SaveUser(&user);
    user
      
}

fn SaveUser(user: &Users) {
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

pub fn InitialCreate() -> Users {
    let user = Users{
        accounts: Vec::new(),
        characters: Vec::new(),
        bots: Vec::new(),
    };
    user
}

pub struct Users{
    pub accounts: Vec<String>,
    pub characters: Vec<String>,
    pub bots: Vec<bool>,
}
