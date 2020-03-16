use std::fs::{File};
use std::process::Command;
use std::io::Write;


pub fn StartCharacters(accounts: &Vec<String>, characters: &Vec<String>, bots: &Vec<String>){
        writeToBatch(&accounts, &characters);
        startMainBatch();
    
}


fn writeToBatch(accounts: &Vec<String>, characters: &Vec<String>){
    let mut file = File::create("characters.bat")
            .expect("unable to write to characters batch");
    let mut i = 0;
    for character in characters.iter(){
        let data = format!("tasklist /nh /fi \"WINDOWTITLE eq {}\" | find /i \"eqgame.exe\" > nul ||(start \"{}\" /d \"C:\\games\\Everquest\\TGCMAIN\\everquest_rof2\" \"C:\\games\\Everquest\\TGCMAIN\\everquest_rof2\\eqgame.exe\" patchme -h /login:{})", character, character, accounts[i]);
        writeln!(file, "{}", data);
        i+=1;
        if i%5 == 0 { writeln!(file, "PING localhost -n 5 >NUL"); }
        if i%6 == 0 { println!("Starting group {}", i/6); }
    }
    writeln!(file, "exit");

}

fn startMainBatch(){
    let command = "start characters.bat";
    println!("{}", command);
    Command::new("cmd")
        .args(&["/C", command])
        .spawn()
        .expect("failed to execute process");
}
