use std::fs::{File};
use std::process::Command;
use std::io::Write;


pub fn StartCharacters(accounts: &Vec<String>, characters: &Vec<String>, bots: &Vec<String>){
        writeToBatch(&accounts, &characters);
        startMainBatch();

}


fn writeToBatch(accounts: &Vec<String>, characters: &Vec<String>){
    let mut file = File::create("characters.sh")
            .expect("unable to write to characters shell");
    let mut i = 0;
    for character in characters.iter(){
        let data = format!("wine ~/eqemu/everquest_rof2/eqgame.exe patchme /login:{}", accounts[i]);
        writeln!(file, "{}", data);
        i+=1;
        //if i%5 == 0 { writeln!(file, "PING localhost -n 5 >NUL"); }
        if i%6 == 0 { println!("Starting group {}", i/6); }
    }
    writeln!(file, "exit");

}

fn startMainBatch(){
    let command = "./characters.sh";
    println!("{}", command);
    Command::new("bash")
        .args(&["-c", command])
        .spawn()
        .expect("failed to execute process");
}
