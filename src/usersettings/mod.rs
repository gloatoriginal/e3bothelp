mod userfilefunctions;

use userfilefunctions::SaveUser;
use userfilefunctions::UserRead;

//create user function start
pub fn UserCreate(GetInput: &dyn Fn() -> String, mut user: Users) -> Users{
    //create mutable strings to assign for user settings struct
    println!("Please enter your Account name");
    let accInput = GetInput();
    println!("Please enter your Character name");
    let charInput = GetInput();
    if user.accounts.iter().any(|i| i == &accInput.trim()) || user.characters.iter().any(|i| i == &charInput.trim()){
        println!("Sorry that account or character already exists");
        user
    } else {
        user.accounts.push(accInput.to_string());
        user.characters.push(charInput.to_string());
        println!("Is this going to be a Bot Character? y or n default is y.");
        if GetInput().trim() == "n" { user.bots.push("false".to_string()); }
        else { user.bots.push("true".to_string()); }
        SaveUser(&user);
        user
    }
}//create user function end

//read user settings files
pub fn ReadUser() -> Users {
    let user = Users {
        accounts: UserRead("accounts".to_string()),
        characters: UserRead("characters".to_string()),
        bots: UserRead("bots".to_string()),
    };
    user
}//read user settings files

//display characters
pub fn Display(user: &Users){
    let mut i = 0;
    for account in user.accounts.iter() {
        println!("Account: {} Character: {} Am I a Bot? {}", account.trim(),
                                            user.characters[i].trim(), user.bots[i].trim());
        i+=1;
    }
}
//display characters


pub struct Users{
    pub accounts: Vec<String>,
    pub characters: Vec<String>,
    pub bots: Vec<String>,
}
