use std::collections::HashMap;

mod tokens;
#[derive(Debug)]
pub struct Mprosser{
    input:String,
    cur_char:String,
    cur_pos:usize,
    difendes:HashMap<String,String>,
}
impl Mprosser{
    pub fn new(input:String)->Self{
        return Self{
            input: input.clone(),
            cur_char: input.chars().nth(0).unwrap().to_string(),
            cur_pos:0,
            difendes:HashMap::new()
        };
    }
    pub fn next(&mut self){
        if self.cur_pos < self.input.len(){
            self.cur_pos += 1;
            self.cur_char = self.input.chars().nth(self.cur_pos).unwrap_or('\0').to_string();
        }
    }
    pub fn make(&mut self)->String{
        let mut out:String = String::new();
        while self.cur_pos < self.input.len(){
            match self.cur_char.as_str(){
                tokens::COMMENT_DEF =>{
                    while self.cur_char !=  "\n"{
                        self.next();
                    }
                    self.next()
                },
                tokens::MACRO_DEF =>{
                    const IMPORT:usize = 0;
                    const DEFINE:usize = 1;

                    self.next();

                    let mut macro_type = String::new();
                    let mut macro_value = String::new();
                    let splitters = [" ","\n"];

                    {
                        let refrs = [&mut macro_type , &mut macro_value];
                        for i in 0..2{
                            while self.cur_char !=  splitters[i] {
                                refrs[i].push_str( self.cur_char.as_str() );

                                self.next();
                            }
                        }
                    }
                    if macro_type == tokens::MACROS[IMPORT]{
                        let path = macro_value.trim();
                        let code = std::fs::read_to_string(path).unwrap();
                        let mut p = Mprosser::new(code.clone());
                        let output = p.make();
                        out.push_str(&output);
                        out.push('\n');
                    }
                    else if macro_type == tokens::MACROS[DEFINE] {
                        let splited = macro_value.split("=");
                        let from = splited.clone().collect::<Vec<&str>>()[0];
                        let to = splited.collect::<Vec<&str>>()[1];
                        self.difendes.insert(from.to_string(), to.to_string());
                    }
                    self.next();
                },
                _=>{out.push_str(self.cur_char.as_str());self.next();}
            }
            
        }
        for i in &self.difendes{
            out = out.replace(i.0.as_str(), i.1.as_str());
        }

        return out;
    }
}