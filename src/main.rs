use std::fmt;
use std::io;
#[derive(Clone)]
enum Space {
    Full(Color),
    Empty,
}
impl Space {
    fn is_empty(&self) -> bool {
        match self {
            Space::Empty => true,
            Space::Full(_) => false,
        }
    }
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
        let empty_row = vec![Space::Empty; width as usize];
        let mut spaces = vec![];
        for _ in 0..height {
            spaces.push(empty_row.clone());
        }
        let board = Board { spaces, width, height };
        board
    }
}
impl Printable for Board {
    fn print(&self) -> () {
        for row in self.spaces.iter().rev() {
            for space in row {
                print!("{} ", space);
            }
            println!("");
        }
    }
}
impl Board {
    fn make_move(&mut self, move_choice: u8, white_trn: bool) -> Result<bool, &str> {
        let height = self.stck_height(move_choice);
        if height == self.height - 1 {
            return Err("That column is full!");
        }
        self.spaces[(height) as usize][(move_choice - 1) as usize] = match white_trn {
            true => Space::Full(Color::White),
            false => Space::Full(Color::Black),
        };
        
        //check if its a winner
        Ok(false)
    }
    fn stck_height(&self, move_choice: u8) -> u8 {
        let mut i = 0;
        while !self.spaces[i][(move_choice - 1) as usize].is_empty()  && 
            i < (self.height - 1) as usize {
            i += 1;
        }
        i.try_into().unwrap()
    }

}
struct Board {
   spaces: Vec<Vec<Space>>,
   width: u8,
   height: u8,
}
struct Game {
    board : Board,
    white_trn : bool,
    trn_cnt : u8,
}
impl Game {
    fn new() -> Game {
        Game { board: Board::new(8, 8), white_trn: true, trn_cnt: 0 }
    }
    fn get_move(&self) -> u8 {
        let width = self.board.width;
        loop {
            println!("Enter a move! (integer between 1 and {})", width);
            let mut s = String::new();
            io::stdin().read_line(&mut s)
                .expect("read failed!");
            let move_choice = match s.trim().parse() {
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
                return move_choice;
            }
        }
    }
    fn do_turn(&mut self) -> () {
        let move_choice = self.get_move();
        match self.board.make_move(move_choice, self.white_trn) {
            Ok(_) => (),
            Err(s) => println!("{}", s),
        };
        self.white_trn = !self.white_trn;
        self.trn_cnt += 1;
    }
}
fn main() {
    let mut game = Game::new();
    game.board.print();
    loop {
        game.do_turn();
        game.board.print();
    }
}

