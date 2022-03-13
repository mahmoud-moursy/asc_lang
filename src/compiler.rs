use std::collections::HashMap;

use crate::tokens::Token;



pub fn compile(code: Vec<Token>) -> Vec<u8> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut compiled_out: Vec<u8> = vec![];

    let mut code = code.into_iter();

    while let Some(token) = code.next() {
        match token {
            Token::Ident(inst) => {
                match inst.as_str() {
                    "bytes" => {
                        while let Some(Token::Byte(byte)) = code.next() {
                            compiled_out.push(byte)
                        }
                    }
                    "noop" => {
                        compiled_out.push(0x00)
                    }
                    "cpix" => {
                        let Some(Token::Byte(x)) = code.next() else {
                            panic!("Unexpected token in cpix")
                        };
                        let Some(Token::Byte(y)) = code.next() else {
                            panic!("Unexpected token in cpix")
                        };
                        let Some(Token::Byte(colour_code)) = code.next() else {
                            panic!("Unexpected token in cpix")
                        };

                        compiled_out.extend(
                            [
                                0x01,
                                x,
                                y,
                                colour_code
                            ]
                        )
                    }
                    "pix" => {
                        let Some(Token::Var(x)) = code.next() else {
                            panic!("Unexpected token in cpix")
                        };
                        let Some(Token::Var(y)) = code.next() else {
                            panic!("Unexpected token in cpix")
                        };
                        let Some(Token::Byte(colour_code)) = code.next() else {
                            panic!("Unexpected token in cpix")
                        };

                        compiled_out.extend(
                            [
                                0x02,
                                x,
                                y,
                                colour_code
                            ]
                        )
                    }
                    "var" => {
                        compiled_out.push(0xa1);

                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in var")
                        };

                        let tok = code.next().unwrap();

                        match tok {
                            Token::Str(str) => {
                                compiled_out.push(
                                    0xab,
                                );
                                compiled_out.extend(str.map(|c| c as u8));
                                compiled_out.push(addr);
                            }
                            Token::Num(num) => {
                                compiled_out.push(0xe0);
                                compiled_out.extend(num.to_le_bytes());
                                compiled_out.push(addr)
                            }
                            Token::Float(float) => {
                                compiled_out.push(0xf0);
                                compiled_out.extend(float.to_le_bytes());
                                compiled_out.push(addr);
                            }
                            Token::Array(arr) => {
                                compiled_out.push(0x8a);
                                compiled_out.extend(arr);
                                compiled_out.push(addr);
                            }
                            any => panic!("Invalid variable token `{any:?}`")
                        }
                    }
                    "let" => {
                        compiled_out.push(0xa2);

                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in var")
                        };

                        let tok = code.next().unwrap();

                        match tok {
                            Token::Str(str) => {
                                compiled_out.push(
                                    0xab,
                                );
                                compiled_out.extend(str.map(|c| c as u8));
                                compiled_out.push(addr);
                            }
                            Token::Num(num) => {
                                compiled_out.push(0xe0);
                                compiled_out.extend(num.to_le_bytes());
                                compiled_out.push(addr)
                            }
                            Token::Float(float) => {
                                compiled_out.push(0xf0);
                                compiled_out.extend(float.to_le_bytes());
                                compiled_out.push(addr);
                            }
                            Token::Array(arr) => {
                                compiled_out.push(0x8a);
                                compiled_out.extend(arr);
                                compiled_out.push(addr);
                            }
                            any => panic!("Invalid variable token `{any:?}`")
                        }
                    }
                    "fdiv" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Float(lhs), Token::Float(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs / rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf0,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "fsub" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Float(lhs), Token::Float(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs - rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf1,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "fadd" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Float(lhs), Token::Float(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs + rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf2,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "fmul" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Float(lhs), Token::Float(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs * rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf3,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "div" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Num(lhs), Token::Num(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs / rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf4,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "sub" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Num(lhs), Token::Num(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs - rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf5,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "add" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Num(lhs), Token::Num(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs + rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf6,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "mul" => {
                        let lhs = code.next().unwrap();
                        let rhs = code.next().unwrap();

                        let out = code.next().unwrap();

                        match (lhs, rhs, out) {
                            (Token::Num(lhs), Token::Num(rhs), Token::Var(out)) => {
                                compiled_out.push(0xa1); 
                                compiled_out.extend((lhs * rhs).to_le_bytes());
                                compiled_out.push(out);
                            }
                            (Token::Var(lhs_addr), Token::Var(rhs_addr), Token::Var(out)) => {
                                compiled_out.extend([
                                    0xf7,
                                    lhs_addr,
                                    rhs_addr,
                                    out
                                ]);
                            }
                            _ => panic!("Unexpected args in fdiv; (fdiv can only accept pairs of constant numbers, or pairs of variables as input)")
                        }
                    }
                    "fjmp" => {
                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in fjmp")
                        };

                        let Some(Token::Ident(jmp_label)) = code.next() else {
                            panic!("Unexpected token in fjmp")
                        };

                        compiled_out.extend([
                            0xe2,
                            addr,
                        ]);

                        compiled_out.extend(labels.get(&jmp_label).unwrap().to_le_bytes())
                    }
                    "tjmp" => {
                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in fjmp")
                        };

                        let Some(Token::Ident(jmp_label)) = code.next() else {
                            panic!("Unexpected token in fjmp")
                        };

                        compiled_out.extend([
                            0xe1,
                            addr,
                        ]);

                        compiled_out.extend(labels.get(&jmp_label).unwrap().to_le_bytes())
                    }
                    "jmp" => {
                        let Some(Token::Ident(jmp_label)) = code.next() else {
                            panic!("Unexpected token in fjmp")
                        };

                        compiled_out.push(0xe3);
                        compiled_out.extend(labels.get(&jmp_label).unwrap().to_le_bytes());
                    }
                    "bjmp" => {
                        let Some(Token::Num(jmp_byte)) = code.next() else {
                            panic!("Unexpected token in fjmp")
                        };

                        compiled_out.push(0xe3);
                        compiled_out.extend(jmp_byte.to_le_bytes());
                    }
                    any => panic!("Unknown instruction: {any}")
                }
            },
            Token::Label(name) => {
                labels.insert(name, compiled_out.len());
            },
            Token::Str(_) => todo!(),
            Token::Byte(_) => todo!(),
            Token::Var(_) => todo!(),
            Token::Num(_) => todo!(),
            Token::Float(_) => todo!(),
            Token::Array(_) => todo!(),
            Token::Block(_) => todo!(),
            Token::EndL => {},
        }
    }

    compiled_out
}