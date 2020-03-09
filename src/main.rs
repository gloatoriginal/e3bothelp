mod mainmenu;
mod saveusersettings;
use saveusersettings::UserCreate;
use saveusersettings::CreateEmptyUsers;
mod generalfunctions;

fn main(){ 
    let staticUser = CreateEmptyUsers();
    while true{
        let whatToDo = mainmenu::MainMenu(&generalfunctions::GetInput);
        if whatToDo == 1 {
           println!("Coming soon"); 
        }
        if whatToDo == 2 { 
            UserCreate(&generalfunctions::GetInput, staticUser);
        } else { break; }
    }
}
