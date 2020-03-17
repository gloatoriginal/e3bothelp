#![allow(non_snake_case)]
#![allow(while_true)]


//mod displayaccounts;
mod mainmenu;
mod generalfunctions;
mod usersettings;
mod win;
mod linux;

use generalfunctions::GetInput;
use mainmenu::MainMenu;

fn main(){
    //create mutable user to make Users struct
    let mut user = usersettings::ReadUser();
    //create a forever while loop
    while true {
        //this is the main menu that pops up
        let whatToDo = MainMenu(&GetInput);
        //this is going to show saved characters/accounts
        if whatToDo == 1 { usersettings::Display(&user); }
        //start of creating users
        else if whatToDo == 2 { user = usersettings::UserCreate(&GetInput, user); }
        //end of creating users
        //start of setting directory settings
        else if whatToDo == 3 { user = usersettings::SetDirectory(&GetInput, user) }

        //this will start the characters(still in production)
        else if whatToDo == 4 {
            if cfg!(target_os = "windows"){ 
                win::StartCharacters(&user.accounts, &user.characters, &user.bots,
                                                        &user.maindir, &user.botdir); 
            }
            else if cfg!(target_os = "linux"){ 
                linux::StartCharacters(&user.accounts, &user.characters, &user.bots); 
            }
            else { println!("Coming soon!"); }

        }
        else { break; }
    }
}
