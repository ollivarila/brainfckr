use crate::lexer::Token;

struct Memory {
    memory: Vec<u8>,
    pointer: usize,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            memory: vec![0; 30_000],
            pointer: 0,
        }
    }
}

pub fn interpret(tokens: Vec<Token>) {
    let mut env = Memory::new();
    let mut instruction: usize = 0;

    while instruction < tokens.len() {
        let token = &tokens[instruction];
        instruction += 1;
        match token {
            Token::IncP => {
                if env.pointer == 29_999 {
                    env.pointer = 0;
                } else {
                    env.pointer += 1;
                }
            }
            Token::DecP => {
                if env.pointer == 0 {
                    env.pointer = 29_999;
                } else {
                    env.pointer -= 1;
                }
            }
            Token::Inc => {
                let val = env.memory[env.pointer];
                if val == 255 {
                    env.memory[env.pointer] = 0;
                } else {
                    env.memory[env.pointer] += 1;
                }
            }
            Token::Dec => {
                let val = env.memory[env.pointer];
                if val == 0 {
                    env.memory[env.pointer] = 255;
                } else {
                    env.memory[env.pointer] -= 1;
                }
            }
            Token::Output => {
                print!("{}", env.memory[env.pointer] as char);
            }
            Token::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                env.memory[env.pointer] = input.as_bytes()[0];
            }
            Token::JumpIfZero(index) => {
                let data = env.memory[env.pointer];
                if data == 0 {
                    instruction = *index
                }
            }
            Token::JumpIfNonZero(index) => {
                let data = env.memory[env.pointer];
                if data != 0 {
                    instruction = *index
                }
            }
        }
    }
}
