use std::str::CharIndices;

#[derive(Debug, Copy, Default, Clone)]
pub struct P {
    pub x: usize,
    pub y: usize,
}

pub type Pos = P;

#[derive(Debug, Clone)]
pub enum Tok {
    Add,
    Div,
    Tab,
    Linefeed,
}

pub type Spanned<Tok, Pos, Error> = Result<(Pos, Tok, Pos), Error>;

#[derive(Debug, Copy, Clone)]
pub enum LexicalError {
    // Not possible
    Sub,
    // Not possible
    Div,
    // Other
    Other,
    // Other2
    Other2(Pos),
}

#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    pos: Pos,
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { 
            pos: Pos::default(),
            chars: input.char_indices(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, Pos, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {

            match self.chars.next() {
                Some((_, '-')) => return Some(Err(LexicalError::Sub)),
                Some((i, '+')) => return Some(Ok((self.pos, Tok::Add, self.pos))),
                Some((i, '/')) => return Some(Ok((self.pos, Tok::Div, self.pos))),
                Some((_, ' ')) => continue ,
                Some((i, '\t')) => return Some(Ok((self.pos, Tok::Tab, self.pos))),
                Some((i, '\n')) => {
                    self.pos.y += 1;
                    return Some(Ok((self.pos, Tok::Linefeed, self.pos)))
                },

                None => return None, // End of file
                _ => return Some(Err(LexicalError::Other)),
            }
        }
    }
}
