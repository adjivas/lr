#[derive(Debug, Copy, Clone)]
pub enum Stmt {
    Add,

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
