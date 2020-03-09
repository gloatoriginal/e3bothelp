mod mainmenu;
mod saveusersettings;
mod generalfunctions;

fn main(){ 
    while true{
        let whatToDo = mainmenu::MainMenu(&generalfunctions::GetInput);
        if whatToDo == 1 {
           println!("Coming soon"); 
        }
        if whatToDo == 2 { 
            let user = saveusersettings::UserCreate(&generalfunctions::GetInput);
        } else { break; }
    }
}
