use std::fmt;
use std::io;
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
impl Board {
    
    fn top_left(&self) -> &Space {
        &self.spaces[0][0]
    }
}
struct Board {
   spaces: Vec<Vec<Space>>,
   width: u8,
   height: u8,
}
struct Game {
    board : Board,
    whiteTrn : bool,
    trnCnt : u8,
}
impl Game {
    fn new() -> Game {
        let mut board = Board::new(8, 8);
        let mut whiteTrn = true;
        let mut trnCnt = 0;
        Game { board, whiteTrn, trnCnt }
    }
    fn get_move(&self) -> u8 {
        let width = self.board.width;
        let mut move_choice = 0;
        loop {
            println!("Enter a move! (integer between 1 and {})", width);
            let mut s = String::new();
            io::stdin().read_line(&mut s)
                .expect("read failed!");
            move_choice = match s.trim().parse() {
                Ok(x) => match x {
                    1..=8 => x,
                    _ => {
                        println!("number out of range! must be between 1 and {}", 
                                 self.board.width);
                        continue;
                    },
                },
                Err(_) => { 
                    println!("input not a valid number!");
                    continue;
                },
            };
            if move_choice != 0 {
                break;
            }
        }
        move_choice
    }
}
fn main() {
    let mut game = Game::new();
    let x = game.get_move(); 
    println!("choice was {}", x);
}

