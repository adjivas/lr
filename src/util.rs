use std::io::prelude::*;
use std::io;
use std::fmt;

pub struct Screen {
  cursor: io::Cursor<Vec<u8>>,
}

impl Screen {

  pub fn new(buf: Vec<u8>) -> Self {
    Screen {
      cursor: io::Cursor::new(buf),
    }
  }

  pub fn goto(&mut self, x: u64, y: u64) -> Option<()> {
    if let Some(index) = x.checked_mul(y) {
        Some(self.cursor.set_position(index))
    } else {
        None
    }
  }

  pub fn push(&mut self, term: u8) -> Result<usize, io::Error> {
    self.cursor.write(&[term])
  }
}

impl fmt::Debug for Screen {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.cursor.get_ref())
  }
}

impl Clone for Screen {
  fn clone(&self) -> Screen {
    Screen {
      cursor: self.cursor.clone(),
    }
  }
  
  fn clone_from(&mut self, source: &Self) {
    self.cursor.write_all(source.cursor.get_ref()).unwrap()
  }
}

impl Default for Screen {
  fn default() -> Screen {
    Screen {
        cursor: io::Cursor::new(Vec::new()),
    }
  }
}
