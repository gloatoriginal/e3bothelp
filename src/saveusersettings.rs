pub fn CreateEmptyUsers() -> Users {
    let users = Users {
        accounts: Vec::new(),
        characters: Vec::new(),
        bots: Vec::new(),
    };
    users
}

pub fn UserCreate(GetInput: &dyn Fn() -> String, users: Users){
    //create mutable strings to assign for user settings struct
   
    println!("Please enter your Account name");
    let a = GetInput();
    println!("Please enter your Character name");
    let c = GetInput();
    println!("Is this going to be a Bot Character? y or n default is y.");
    let mut b = true;
    let botCheck = GetInput();
    if botCheck.trim() == "n" { b = false; }
    else { b = true; }

    let user = User {
        account: String::from(a),
        character: String::from(c),
        bot: b,
    };
    UserAdd(user, users);     
}

struct User{
    account: String,
    character: String,
    bot: bool,
}

fn UserAdd(user: User, mut users: Users){
    users.accounts.push(user.account); 
    users.characters.push(user.character);
    users.bots.push(user.bot);
}

pub struct Users{
    accounts: Vec<String>,
    characters: Vec<String>,
    bots: Vec<bool>,
}
