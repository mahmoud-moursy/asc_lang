#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Label(String),
    Str([char; 8]),
    Var(u8),
    Num(i64),
    Byte(u8),
    Float(f64),
    Array([u8; 8]),
    Block(Vec<Token>),
    EndL,
}

impl Token {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Token::Ident(_) => panic!("Impossible to convert into bytes!"),
            Token::Label(_) => panic!("Impossible to convert into bytes!"),
            Token::Str(v) => Vec::from(v.map(|c| c as u8)),
            Token::Byte(v) => vec![v],
            Token::Var(v) => vec![v],
            Token::Num(v) => Vec::from(v.to_le_bytes()),
            Token::Float(v) => Vec::from(v.to_le_bytes()),
            Token::Array(_) => todo!(),
            Token::Block(_) => panic!("Impossible to convert into bytes!"),
            Token::EndL => panic!("Impossible to convert into bytes!"),
        }
    }
}