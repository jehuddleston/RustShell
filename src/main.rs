use std::io::{self, BufRead,Write,stdout};

mod commandmanager;
use crate::commandmanager::CommandManager; 

fn reply(args:Vec<String>) -> String{
    String::from("return")
}

fn main(){
    let mut check = true;
    let mut cm = CommandManager::new();
    cm.register("reply",reply);
    while check{
        print!(">>");
        stdout().flush();
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut buffer).unwrap();
        println!("Read in {}", buffer);
        if buffer == "quit\n"
        {check = false;}
    }
}

