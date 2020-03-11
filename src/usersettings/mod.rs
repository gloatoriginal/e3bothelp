mod saveuser;
mod readuser;

use saveuser::SaveUser;
use readuser::ReadUser;

//create user function
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


pub fn InitialCreate() -> Users {
    ReadUser()
}

pub struct Users{
    pub accounts: Vec<String>,
    pub characters: Vec<String>,
    pub bots: Vec<bool>,
}
