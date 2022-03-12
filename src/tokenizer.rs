use crate::tokens::Token;

pub fn tokenize(file: String) -> Vec<Token> {
    let mut file = file.chars().peekable();

    let mut out = vec![];

    while let Some(chr) = file.next() {
        match chr {
            '/' => {
                while Some('/') != file.next() {}
            }
            'a'..='z' => {
                let mut string = String::from(chr);

                while let Some(chr @ 'a'..='z') = file.next() {
                    string.push(chr)
                }

                out.push(Token::Ident(string))
            }
            '$' => {
                let mut hex = String::new();

                while let Some('a'..='f' | 'A'..='F' | '0'..='9') = file.peek() {
                    hex.push(file.next().unwrap());
                }

                let num = i64::from_str_radix(&hex, 16).unwrap();

                if num > 255 {
                    panic!("Var accessor value out of bounds (Value was {num:x}, max is {:x})", 255)
                }

                out.push(Token::Var(num as u8))
            }
            '@' => {
                let mut hex = String::new();

                while let Some('a'..='f' | 'A'..='F' | '0'..='9') = file.peek() {
                    hex.push(file.next().unwrap());
                }

                out.push(Token::Num(i64::from_str_radix(&hex, 16).unwrap()))
            }
            '#' => {
                let mut float = String::new();

                while let Some('0'..='9' | '.') = file.peek() {
                    float.push(file.next().unwrap())
                }

                out.push(Token::Float(float.parse().unwrap()))
            }
            '0'..='9' => {
                let mut num = String::from(chr);

                while let Some('0'..='9') = file.peek() {
                    num.push(file.next().unwrap());
                }

                out.push(Token::Num(num.parse().unwrap()))
            }
            '"' => {
                let mut string = ['\u{0}'; 8];

                let mut count: usize = 0;

                while let Some(chr) = file.next() && chr != '"' {
                    string[count] = chr;
                    count += 1;
                    if count == 8 {
                        panic!("String overflow (You inputted a string that was over eight characters in length)")
                    }
                }

                out.push(Token::Str(string))
            }
            '{' => {
                let mut open_brackets = 1;

                let mut to_eval = String::new();

                while open_brackets != 0 {
                    let Some(ch) = file.next() else {
                        panic!("Unexpected EOF")
                    };

                    match ch {
                        '}' => {
                            open_brackets -= 1;
                            if open_brackets == 0 {
                                continue
                            }
                            to_eval.push(chr)
                        },
                        any => to_eval.push(any)
                    }
                }

                out.push(Token::Block(tokenize(to_eval)))
            }
            '!' => {
                let mut label = String::new();

                while let Some('a'..='z') = file.peek() {
                    label.push(file.next().unwrap())
                }

                out.push(Token::Label(label))
            }
            ';' => out.push(Token::EndL),
            a if a.is_whitespace() => {}
            _   => panic!("Unexpected char {chr}")
        }
    }

    out
}