pub const KEYWORDS :[&str;4] = ["let","fn","while","if","int","string","struct"];
#[derive(Debug)]
pub enum Token{
    Add,
    SemiColon,
    Macro(String),
    Mul,
    Min,
    Div,
    Rb,
    Lb,
    Be,
    And,
    Or,
    Xor,
    Keyword(String),
    Name(String),
    Number(i32),
}
