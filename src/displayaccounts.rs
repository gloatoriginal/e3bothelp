use crate::usersettings::Users;


pub fn Display(user: &Users){
    let mut i = 0;
    for account in user.accounts.iter() {
        println!("Account: {} Character: {} Am I a Bot? {}", account.trim(),
                                            user.characters[i].trim(), user.bots[i]);
        i+=1;
    }

}
