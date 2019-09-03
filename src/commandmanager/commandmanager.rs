use std::collections::HashMap;

type Exec = fn(Vec<String>) -> String;

pub struct CommandManager{
    //commands: HashMap<String,dyn Fn(&Vec<String>)>,
    commands: HashMap<String,Exec>,
}

impl CommandManager {
    pub fn new() -> Self{
        let cm = CommandManager{
            commands:HashMap::new(),
        };
        cm
    }

    pub fn register(&mut self,command: &str, f: Exec) {
       // let strcopy = command;
         let ret = self.commands.insert(command.to_string(),f);
         if ret != None {
            eprintln!("couldn't register {}: command already in use", command);
         }
    }

    pub fn execute(&self, command: &str, args: Vec<String>) -> String{

        let f = self.commands.get(&command.to_string());
        if f == None {
            return String::from("ERROR: Command does not exist");
        }
        let f = f.unwrap();
        let ret = f(args);
        ret
    }

} 
