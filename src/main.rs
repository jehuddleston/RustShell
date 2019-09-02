use std::io::{self, BufRead};

fn main(){
    let mut check = true;

    while check{
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut buffer).unwrap();
        println!("Read in {}", buffer);
        if buffer == "quit\n"
        {check = false;}
    }
}

