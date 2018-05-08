use std::str::CharIndices;

pub type Loc = usize;

#[derive(Debug, Clone)]
pub enum Tok {
    Add,
    Div,
    Tab,
    Linefeed,
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Copy, Clone)]
pub enum LexicalError {
    // Not possible
    Sub,
    // Not possible
    Div,
    // Other
    Other,
}

#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, '-')) => return Some(Err(LexicalError::Sub)),
                Some((i, '+')) => return Some(Ok((i, Tok::Add, i+1))),
                Some((i, '/')) => return Some(Ok((i, Tok::Div, i+1))),
                Some((_, ' ')) => continue ,
                Some((i, '\t')) => return Some(Ok((i, Tok::Tab, i+1))),
                Some((i, '\n')) => return Some(Ok((i, Tok::Linefeed, i+1))),

                None => return None, // End of file
                _ => return Some(Err(LexicalError::Other)),
            }
        }
    }
}
