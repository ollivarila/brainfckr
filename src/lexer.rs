#[derive(Debug, Clone)]
pub enum Token {
    IncP,
    DecP,
    Inc,
    Dec,
    Input,
    Output,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}

#[derive(Clone)]
struct Source {
    source: String,
    index: usize,
}

impl Source {
    fn new(source: String) -> Source {
        let source = source.chars().filter(is_bf).collect();
        Source { source, index: 0 }
    }
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }
    fn consume(&mut self) -> Option<char> {
        let current = self.peek();
        self.index += 1;
        current
    }
    fn at_end(&self) -> bool {
        self.index >= self.source.len()
    }
}

fn is_bf(c: &char) -> bool {
    let bf = "<>-+[].,";
    bf.contains(*c)
}

pub fn parse(source: impl Into<String>) -> Vec<Token> {
    let mut code = Source::new(source.into());
    let mut tokens = Vec::new();

    while !code.at_end() {
        match code.consume() {
            Some('>') => tokens.push(Token::IncP),
            Some('<') => tokens.push(Token::DecP),
            Some('+') => tokens.push(Token::Inc),
            Some('-') => tokens.push(Token::Dec),
            Some(',') => tokens.push(Token::Input),
            Some('.') => tokens.push(Token::Output),
            Some('[') => tokens.push(parse_jump_start(&code)),
            Some(']') => tokens.push(parse_jump_end(&code)),
            _ => {}
        }
    }

    tokens
}

fn parse_jump_start(source: &Source) -> Token {
    let mut source = source.clone();
    let mut open = vec![source.index - 1];
    let mut close = vec![];

    while !source.at_end() {
        let c = source.consume();
        match c {
            Some(']') => {
                close.push(source.index);
                if open.len() == close.len() {
                    let index = close.pop().unwrap();
                    return Token::JumpIfZero(index);
                }
            }
            Some('[') => open.push(source.index),
            _ => {}
        }
    }

    panic!("Loop is messed up (Missing `]`)")
}

fn parse_jump_end(source: &Source) -> Token {
    let mut open = vec![];
    let mut close = vec![];
    let mut i = source.index - 1;

    let chars: Vec<char> = source.source.chars().collect();

    loop {
        let c = chars.get(i).expect("Could not find matching `[`");
        match c {
            '[' => {
                open.push(i);
                if open.len() == close.len() {
                    let index = open.pop().unwrap();
                    return Token::JumpIfNonZero(index);
                }
            }
            ']' => close.push(i),
            _ => {}
        }
        i -= 1;
    }
}
