use std::fmt;
use rand::Rng;
#[derive(Clone)]
enum Space {
    Full(Color),
    Empty,
}
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Space::Full(c) => {
                match c {
                    Color::Black => write!(f, "b"),
                    Color::White => write!(f, "w"),
                }
            }
            Space::Empty => write!(f, "e"),
        }
    }
}
#[derive(Clone)]
enum Color {
    Black,
    White,
}
trait Rectangular {
    fn new (width: u8, height: u8) -> Self;
}
trait Printable {
    fn print (&self) -> ();
}

impl Rectangular for Board {
    fn new (width: u8, height: u8) -> Board {
        let mut empty_row = vec![Space::Empty; width as usize];
        let mut spaces = vec![];
        for i in 0..height {
            spaces.push(empty_row.clone());
        }
        let mut board = Board { spaces, width, height };
        board
    }
}
impl Printable for Board {
    fn print(&self) -> () {
        for row in &self.spaces {
            for space in row {
                print!("{} ", space);
            }
            println!("");
        }
    }
}

struct Board {
   spaces: Vec<Vec<Space>>,
   width: u8,
   height: u8,
}
fn main() {
    let mut board = Board::new(5, 5);
    board.print();
}
