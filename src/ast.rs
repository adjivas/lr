#[derive(Debug)]
pub enum Stmt {

    Mark(String),
    Call(String),
    Jump(String), // Unconditional jump
    Jz(String),   // Jump if zero
    Js(String), // Jump if negative

    Return,
    Exit,
}

pub fn label(digits: Vec<u8>) -> String {
    digits
        .into_iter()
        .map(|c| match c {
            0 => 's',
            1 => 't',
            _ => unreachable!(),
        })
        .collect()
}
