#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Label(String),
    Str([char; 8]),
    Var(u8),
    Num(i64),
    Float(f64),
    Array(Vec<Token>),
    Block(Vec<Token>),
    EndL
}

macro_rules! op_match {
    ([$expr: expr] => $($ident: ident),*$(,)?) => {
        #[repr(u16)]
        pub enum Instruction {
            NoOp = 0x0,
            $($ident),*
        }

        impl Token {
        pub fn as_instruction(self) -> Instruction {
            let Token::Ident(id) = self else {
                panic!("Unexpected token {self:?}")
            };

            match id.as_str() {
                "noop" => Instruction::NoOp,
                $(
                    c if c == stringify!($ident).to_lowercase() => Instruction::$ident
                ),*,
                inst => panic!("Unknown instruction {inst:?}")
                }
            }
        }
    };
}

op_match! {
    [id.as_str()] => 
        WriteVar,
        Pix,
        Add,
        Sub,
        Mul,
        Not,
        While,
        If,
}