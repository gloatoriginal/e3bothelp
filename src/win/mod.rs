use std::process::Command;


pub fn StartCharacters(accounts: &Vec<String>, characters: &Vec<String>, bots: &Vec<String>){
    startThis();
}
/*
pub fn StartCharacters(){
    startThis(String::from("cmd"));
}*/


fn startThis(){
    //let command = "tasklist /nh /fi \"WINDOWTITLE eq Gloatt\" |find /i \"eqgame.exe\" > nul ||(start \"Gloatt\" /d \"C:\\games\\Everquest\\TGCMAIN\\everquest_rof2\" \"C:\\games\\Everquest\\TGCMAIN\\everquest_rof2\\eqgame.exe\" patchme -h /login:devwarrior)";
    println!("{}", command);
    Command::new("cmd")
        .args(&["/C", command])
        .spawn()
        .expect("failed to execute process");
}
