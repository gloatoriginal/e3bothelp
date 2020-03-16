use std::fs::{File};
use std::process::Command;
use std::io::Write;


pub fn StartCharacters(accounts: &Vec<String>, characters: &Vec<String>, bots: &Vec<String>){
    for i in accounts.iter(){
        writeToBatch();
        startThis();
    }
    
}


fn writeToBatch(){
    let data = "tasklist /nh /fi \"WINDOWTITLE eq Gloat\" | find /i \"eqgame.exe\" > nul ||(start \"Gloat\" /d \"C:\\games\\Everquest\\TGCMAIN\\everquest_rof2\" \"C:\\games\\Everquest\\TGCMAIN\\everquest_rof2\\eqgame.exe\" patchme -h /login:Devwarrior)";
    let mut file = File::create("characters.bat")
        .expect("unable to create accounts file");
    writeln!(file, "{}", data);
}

fn startThis(){
    let command = "start characters.bat\nexit";
    println!("{}", command);
    Command::new("cmd")
        .args(&["/C", command])
        .spawn()
        .expect("failed to execute process");
}
