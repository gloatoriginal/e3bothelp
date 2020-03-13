use std::process::Command;


pub fn StartCharacters(){
    startThis(String::from("cmd"));
}


fn startThis(command: String){
    let comm = "echo hello";
    let output = Command::new(command)
                        .args(&["/C", comm])
                        .output()
                        .expect("Failed to execute");
    println!("The stdout is {}", output.stdout);
}