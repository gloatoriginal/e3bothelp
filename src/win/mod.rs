use std::fs::{File};
use std::process::Command;
use std::io::Write;
extern crate num_cpus;

pub fn StartCharacters(accounts: &Vec<String>, characters: &Vec<String>, bots: &Vec<String>,
                        mainDir: &String, botDir: &String){
        writeToBatch(&accounts, &characters, &bots, &mainDir, &botDir);
        startMainBatch();
    
}

fn writeToBatch(accounts: &Vec<String>, characters: &Vec<String>, bots: &Vec<String>,  
                mainDir: &String, botDir: &String){
    let mut file = File::create("characters.bat")
            .expect("unable to write to characters batch");
    let mut i = 0;
    for character in characters.iter(){
        if bots[i] == "false"{
        let data = format!("tasklist /nh /fi \"WINDOWTITLE eq {}\" | find /i \"eqgame.exe\" > nul ||(start \"{}\" /d \"{}\" \"{}\\eqgame.exe\" patchme -h /login:{})", 
                            character, character, mainDir.trim(), mainDir.trim(), accounts[i]);
        writeln!(file, "{}", data);
    } else {
        let data = format!("tasklist /nh /fi \"WINDOWTITLE eq {}\" | find /i \"eqgame.exe\" > nul ||(start \"{}\" /d \"{}\" \"{}\\eqgame.exe\" patchme -h /login:{})", 
                            character, character, botDir.trim(), botDir.trim(), accounts[i]);
        writeln!(file, "{}", data);
    }
        
        i+=1;
        if i%5 == 0 { writeln!(file, "PING localhost -n 5 >NUL"); }
        if i%6 == 0 { println!("Starting group {}", i/6); }
    }
    writeln!(file, "PING localhost -n 10 >NUL");
    let data = format!("powershell \"$process=GET-PROCESS eqgame; foreach ($i in $process) {{$i.ProcessorAffinity={}}}\"", 
                        i32::pow(2, (num_cpus::get() as u32))-1);
    writeln!(file, "{}", data);
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