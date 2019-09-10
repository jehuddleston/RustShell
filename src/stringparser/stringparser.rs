use std::str::Split;

pub struct StringParser<'a>{
    splitter: &'a str,
}

impl<'a> StringParser<'a>{
    pub fn new(div: &'a str) -> Self{
        let sp = StringParser{
            splitter: div,
        };
        sp
    }

    pub fn parse_args(&mut self, input: String) -> Vec<String> {

        let mut vec: Vec<String> = Vec::new();
        let split = input.split(self.splitter);
        for s in split {
            vec.push(String::from(s));
        }
        vec
    }
}
