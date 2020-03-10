#![allow(non_snake_case)]
#![allow(while_true)]


mod displayaccounts;
mod mainmenu;
mod generalfunctions;
mod saveusersettings;

use displayaccounts::Display;
use mainmenu::MainMenu;
use generalfunctions::GetInput;
use saveusersettings::UserCreate;
use saveusersettings::InitialCreate;


fn main(){
    //create mutable user to make Users struct
    let mut user = InitialCreate(); 
    //create a forever while loop
    while true {
        let whatToDo = MainMenu(&GetInput);
        if whatToDo == 1 {
           Display(&user);
        } else if whatToDo == 2 {
            //make new user with account/character/bot info
            user = UserCreate(&GetInput, user); 
        } else { break; }
    }
}
