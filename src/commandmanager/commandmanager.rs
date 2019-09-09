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
         let ret = self.commands.insert(command.to_string(),f);
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

#[cfg(test)]
mod test{
    use super::*;
    #[test]

    fn test_constructor() {
        let cm = CommandManager::new();
        assert!(cm.commands.is_empty());
    }

    #[test]
    fn test_add_execute(){
        let mut cm = CommandManager::new();

        fn reply(args: Vec<String>) -> String{
            let mut ret = String::new();
            for s in args{
                ret += &s;
            }
            ret
        };

        cm.register("reply", reply);
        let mut args = Vec::new();
        args.push(String::from("one"));
        args.push(String::from("two"));
        args.push(String::from("three"));
        let response = cm.execute("reply", args);
        assert!(response == String::from("onetwothree"));
                
    }

    #[test]
    fn test_bad_cmd(){
        let cm = CommandManager::new();
        let response = cm.execute("bad", Vec::new());
        assert!(response == String::from("ERROR: Command does not exist"));
    }

    #[test]
    fn test_cmd_already_used(){
        let mut cm = CommandManager::new();

        fn one(args: Vec<String>) -> String{
            String::from("one")
        }
        fn two(args: Vec<String>) -> String{
            String::from("two")
        }

        cm.register("test",one);
        cm.register("test",two);
        let res = cm.execute("test",Vec::new());
        assert!(res == String::from("two"));
    }


}
