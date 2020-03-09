
pub fn UserCreate(GetInput: &dyn Fn() -> String) -> Users {
    //create mutable strings to assign for user settings struct
   
    println!("Please enter your Account name");
    let mut a = GetInput();
    println!("Please enter your Character name");
    let mut c = GetInput();
    let b = true;

    let user = User {
        account: String::from(a),
        character: String::from(c),
        bot: b,
    };
    UserAdd(user)     
}

struct User{
    account: String,
    character: String,
    bot: bool,
}

fn UserAdd(user: User) -> Users {
    let mut accVec = Vec::new();
    let mut chaVec = Vec::new();
    let mut botVec = Vec::new();
    accVec.push(user.account);
    chaVec.push(user.character);
    botVec.push(user.bot);

    let users = Users{
        accounts: accVec,
        characters: chaVec,
        bots: botVec,
   };
    users
}

pub struct Users{
    pub accounts: Vec<String>,
    pub characters: Vec<String>,
    bots: Vec<bool>,
}
