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
use win::StartCharacters;

fn main(){
    //create mutable user to make Users struct
    let mut user = ReadUser();
    //create a forever while loop
    while true {
        let whatToDo = MainMenu(&GetInput);
        if whatToDo == 1 { Display(&user); } 
        else if whatToDo == 2 { user = UserCreate(&GetInput, user); }
        else if whatToDo == 3 {  StartCharacters(); } 
        else { break; }
    }
}
