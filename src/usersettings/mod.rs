mod accountsave;
mod directorysave;


//create user function start
pub fn UserCreate(GetInput: &dyn Fn() -> String, mut user: Users) -> Users{
    //create mutable strings to assign for user settings struct
    println!("Please enter your Account name");
    let accInput = GetInput();
    println!("Please enter your Character name");
    let charInput = GetInput();
    if user.accounts.iter().any(|i| i == &accInput.trim()) || 
        user.characters.iter().any(|i| i == &charInput.trim()){
        println!("Sorry that account or character already exists");
        user
    } else {
        user.accounts.push(accInput.trim().to_string());
        user.characters.push(charInput.trim().to_string());
        accountsave::SaveUser(&user.accounts, "accounts".to_string());
        accountsave::SaveUser(&user.characters, "characters".to_string());
        println!("Is this going to be a Bot Character? y or n default is y.");
        if GetInput().trim() == "n" { user.bots.push("false".to_string()); }
        else { user.bots.push("true".to_string()); }
        accountsave::SaveUser(&user.bots, "bots".to_string());
        user
    }
}//create user function end
//set everquest directory function start
pub fn SetDirectory(GetInput: &dyn Fn() -> String, mut user: Users) -> Users {
    println!("Please paste your main directory");
    user.maindir = GetInput().trim().to_string();
    directorysave::SaveDirectory(&user.maindir, "maindir".to_string());
    println!("Please paste your bot directory");
    user.botdir = GetInput().trim().to_string();
    directorysave::SaveDirectory(&user.botdir, "botdir".to_string());
    println!("updated Main Directory: {} and Bot Directory: {}", 
                                    user.maindir, user.botdir);
    user
}
//set everquest directory function end
//read user settings files
pub fn ReadUser() -> Users {
    let user = Users {
        accounts: accountsave::AccountRead("accounts".to_string()),
        characters: accountsave::AccountRead("characters".to_string()),
        bots: accountsave::AccountRead("bots".to_string()),
        maindir: directorysave::ReadDirectory("maindir".to_string()),
        botdir: directorysave::ReadDirectory("botdir".to_string()),
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
    pub maindir: String,
    pub botdir: String,
}
