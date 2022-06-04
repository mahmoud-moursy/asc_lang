use std::collections::HashMap;

use crate::tokens::Token;



pub fn compile(code: Vec<Token>, compiled_out: &mut Vec<u8>, labels: &mut HashMap<String, usize>, routines: &mut HashMap<String, Vec<Token>>, header_size: &mut usize) {
    let mut code = code.into_iter();

    while let Some(token) = code.next() {

        match token {
            Token::Ident(inst) => {
                match inst.as_str() {
                    "keeploop" => { compiled_out.push(0x02); *header_size += 1; },
                    "keepopen" => { compiled_out.push(0x04); *header_size += 1; },
                    "endhead" => compiled_out.push(0x00),
                    "headerbytes" => {
                        while let Some(Token::Byte(byte)) = code.next() {
                            compiled_out.push(byte);
                            if byte != 0 {
                                *header_size += 1;
                            }
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
                            panic!("Unexpected token in pix")
                        };
                        let Some(Token::Var(y)) = code.next() else {
                            panic!("Unexpected token in pix")
                        };
                        let Some(Token::Byte(colour_code)) = code.next() else {
                            panic!("Unexpected token in pix")
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                                compiled_out.push(0xe0);
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
                    "gt" => {
                        let Some(Token::Var(lhs)) = code.next() else {
                            panic!("Unexpected token in gt call")
                        };

                        let Some(Token::Var(rhs)) = code.next() else {
                            panic!("Unexpected token in gt call")
                        };

                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in gt call")
                        };

                        compiled_out.extend([
                            0xb1,
                            lhs,
                            rhs,
                            addr
                        ]);
                    }
                    "lt" => {
                        let Some(Token::Var(lhs)) = code.next() else {
                            panic!("Unexpected token in lt call")
                        };

                        let Some(Token::Var(rhs)) = code.next() else {
                            panic!("Unexpected token in lt call")
                        };

                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in lt call")
                        };

                        compiled_out.extend([
                            0xb2,
                            lhs,
                            rhs,
                            addr
                        ]);
                    }
                    "key" => {
                        let Some(Token::Byte(keycode)) = code.next() else {
                            panic!("Unexpected token in cin call")
                        };

                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in cin call")
                        };

                        compiled_out.extend([
                            0xd0,
                            keycode,
                            addr
                        ])
                    }
                    "routine" => {
                        let Some(Token::Ident(routine)) = code.next() else {
                            panic!("Unexpected token in routine call")
                        };

                        let Some(Token::Block(block)) = code.next() else {
                            panic!("Unexpected token in routine call")
                        };

                        routines.insert(routine, block);
                    }
                    "call" => {
                        let Some(Token::Ident(routine)) = code.next() else {
                            panic!("Unexpected token in routine invocation")
                        };

                        let block = routines.get(&routine).unwrap();

                        compile(block.clone(), compiled_out, labels, routines, header_size);
                    }
                    "if" => {
                        let Some(Token::Var(addr)) = code.next() else {
                            panic!("Unexpected token in if statement condition")
                        };

                        let Some(Token::Block(block)) = code.next() else {
                            panic!("Unexpected token in if statement (expected block)")
                        };

                        compiled_out.push(0xe2);
                        compiled_out.push(addr);

                        let byte_marker = compiled_out.len();

                        compiled_out.extend([0; 8]);

                        compile(block, compiled_out, labels, routines, header_size);

                        let pos = compiled_out.len().to_le_bytes();

                        for i in byte_marker..byte_marker + 8 {
                            compiled_out[i] = pos[i - byte_marker];
                        }
                    }
                    "rep" => {
                        let Some(Token::Num(i)) = code.next() else {
                            panic!("Only constant ints can be in rep statements!")
                        };

                        let Some(Token::Block(block)) = code.next() else {
                            panic!("Unexpected token in rep statement! (Expected block)");
                        };

                        for _ in 0..i {
                            compile(block.clone(), compiled_out, labels, routines, header_size);
                        }
                    }
                    "spr" => {
                        let Some(Token::Array(arr)) = code.next() else {
                            panic!("Unexpected token in sprite draw call (expected pointer array)")
                        };

                        let Some(Token::Var(x)) = code.next() else {
                            panic!("Unexpected token in sprite draw call (unexpected token passed in as x)")
                        };

                        let Some(Token::Var(y)) = code.next() else {
                            panic!("Unexpected token in sprite draw call (unexpected token passed in as y)")
                        };

                        compiled_out.push(0x03);
                        compiled_out.extend(arr);
                        compiled_out.extend([x, y]);
                    }
                    "flsh" => {
                        compiled_out.push(0xfb)
                    }
                    "cls" => {
                        let Some(Token::Byte(cls)) = code.next() else {
                            panic!("Unexpected token in cls statement!")
                        };

                        compiled_out.extend([0xfc, cls])
                    }
                    any => panic!("Unknown instruction: {any}")
                }
            },
            Token::Label(name) => {
                let len = compiled_out.len() - 1 - *header_size;
                labels.insert(name, len);
            },
            Token::EndL => {},
            any => panic!("Unexpected token {any:?}"),
        }
    }
}