use crate::tokens::Token;

pub fn tokenize(file: String) -> Vec<Token> {
    let mut file = file.chars().peekable();

    let mut out = vec![];

    let mut byte = 0;

    while let Some(chr) = file.next() {
        match chr {
            '/' => {
                while Some('/') != file.next() {
                    byte += 1
                }
                
            }
            'a'..='z' => {
                let mut string = String::from(chr);

                while let Some(chr @ 'a'..='z') = file.next() {
                    string.push(chr);
                    byte += 1;
                }

                out.push(Token::Ident(string))
            }
            '$' => {
                let mut hex = String::new();

                while let Some('a'..='f' | 'A'..='F' | '0'..='9') = file.peek() {
                    hex.push(file.next().unwrap());
                    byte += 1;
                }

                let num = i64::from_str_radix(&hex, 16).unwrap();

                if num > 255 {
                    panic!("Var accessor value out of bounds (Value was {num:x}, max is {:x}) at character {byte}", 255)
                }

                out.push(Token::Var(num as u8))
            }

            '@' => {
                let mut hex = String::new();

                while let Some('a'..='f' | 'A'..='F' | '0'..='9') = file.peek() {
                    hex.push(file.next().unwrap());
                    byte += 1;
                }

                out.push(Token::Num(i64::from_str_radix(&hex, 16).unwrap()))
            }
            '#' => {
                let mut float = String::new();

                while let Some('0'..='9' | '.') = file.peek() {
                    float.push(file.next().unwrap());
                    byte += 1;
                }

                out.push(Token::Float(float.parse().unwrap()))
            }
            '+' => {
                let mut val = String::new();

                for _ in 0..2 {
                    val.push(
                        file.next().unwrap()
                    );
                }

                byte += 2;

                out.push(Token::Byte(
                    u8::from_str_radix(&val, 16).expect(&format!("Expected hexadecimal value in byte declaration at character {byte}"))
                ))
            }
            '0'..='9' | '-' => {
                let mut num = String::from(chr);

                while let Some('0'..='9') = file.peek() {
                    num.push(file.next().unwrap());
                    byte += 1;
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
                        panic!("String overflow (You inputted a string that was over eight characters in length) at character {byte}")
                    }
                    byte += 1;
                }

                out.push(Token::Str(string))
            }
            '{' => {
                let mut open_brackets = 1;

                let mut to_eval = String::new();

                while open_brackets != 0 {
                    let Some(ch) = file.next() else {
                        panic!("Unexpected EOF at character {byte}")
                    };

                    byte += 1;

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
            '[' => {
                let mut open_brackets = 1;

                let mut to_eval = String::new();

                while open_brackets != 0 {
                    let Some(ch) = file.next() else {
                        panic!("Unexpected EOF at character {byte}")
                    };

                    byte += 1;

                    match ch {
                        ']' => {
                            open_brackets -= 1;
                            if open_brackets == 0 {
                                continue
                            }
                            to_eval.push(chr)
                        },
                        any => to_eval.push(any)
                    }
                }

                let arr = tokenize(to_eval);

                if arr.len() != 8 {
                    panic!("Array length can only be 8! Error at character {byte}")
                }

                let mut arr_out = [0u8; 8];

                let _ = arr.iter().enumerate().inspect(|(i, v)| {
                    let Token::Byte(v) = v else {
                        panic!("You can only have bytes in arrays! Error at character {byte}")
                    };

                    arr_out[*i] = *v;
                });

                out.push(Token::Array(arr_out))
            }
            '!' => {
                let mut label = String::new();

                while let Some('a'..='z') = file.peek() {
                    label.push(file.next().unwrap());
                    byte += 1;
                }

                out.push(Token::Label(label))
            }
            ';' => out.push(Token::EndL),
            a if a.is_whitespace() => {  }
            _   => panic!("Unexpected char {chr} at character {byte}")
        }
        byte += 1;
    }

    out
}