#![allow(non_snake_case)]
#![allow(while_true)]


//mod displayaccounts;
mod mainmenu;
mod generalfunctions;
mod usersettings;
mod win;

use mainmenu::MainMenu;
use generalfunctions::GetInput;
use usersettings::UserCreate;
use usersettings::ReadUser;
use usersettings::Display;

fn main(){
    //create mutable user to make Users struct
    let mut user = ReadUser();
    //create a forever while loop
    while true {
        //this is the main menu that pops up
        let whatToDo = MainMenu(&GetInput);
        //this is going to show saved characters/accounts
        if whatToDo == 1 { Display(&user); }
        //start of creating users
        else if whatToDo == 2 { user = UserCreate(&GetInput, user); }
        //end of creating users
        //this will start the characters(still in production)
        else if whatToDo == 3 {
            if cfg!(target_os = "windows"){ win::StartCharacters(&user.accounts, &user.characters, &user.bots); }
            else { println!("Coming soon!"); }
            
        }
        else { break; }
    }
}
