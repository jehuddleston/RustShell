use std::io::{self, BufRead,Write,stdout};

mod commandmanager;
use crate::commandmanager::CommandManager; 
mod stringparser;
use crate::stringparser::StringParser;

fn reply(args:Vec<String>) -> String{
    let mut reply :String = String::new();
    for s in args{
        reply += &s;
    }
    reply
}

fn main(){
    let mut check = true;
    let mut cm = CommandManager::new();
    let mut sp = StringParser::new(" ");
    cm.register("reply",reply);
    while check{
        print!(">>");
        stdout().flush();
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut buffer).unwrap();
        let args = sp.parse_args(buffer);

        let reply = cm.execute("reply",args);
        println!("{}",reply);
        if reply == String::from("quit\n")
        {check = false;}
    }
}

