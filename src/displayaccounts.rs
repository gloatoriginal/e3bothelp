use crate::saveusersettings::Users;


pub fn Display(user: &Users){
    let mut i = 0;
    for account in user.accounts.iter() {
        println!("Account: {}Character: {}Am I a Bot? {}", account, 
                                            user.characters[i], user.bots[i]);
        i+=1;
    }

}
