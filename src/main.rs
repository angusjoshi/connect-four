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
    fn is_same(&self, white_trn: bool) -> bool {
        match self {
            Space::Full(Color::White) => white_trn,
            Space::Full(Color::Black) => !white_trn,
            _ => false,
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
            Space::Empty => write!(f, "_"),
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
        print!("\r");
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
        if height >= self.height {
            return Err("That column is full!");
        }
        self.spaces[(height) as usize][(move_choice - 1) as usize] = match white_trn {
            true => Space::Full(Color::White),
            false => Space::Full(Color::Black),
        };
        
        //check if its a winner
        Ok(self.was_winner(move_choice - 1, height, white_trn))
    }
    fn stck_height(&self, move_choice: u8) -> u8 {
        let mut i = 0;
        while i < (self.height) as usize && !self.spaces[i][(move_choice - 1) as usize].is_empty() {
            i += 1;
        }
        i.try_into().unwrap()
    }
    fn was_winner(&self, move_choice: u8, height: u8, white_trn: bool) -> bool {
        let offsets: Vec<(usize, usize)> = vec![(0,1), (1,0), (1, 1)];
        for (x_offset, y_offset) in &offsets {
            let mut i: usize = height as usize;
            let mut j: usize = move_choice as usize;
            let mut connected = 0;

            //try adding current offset until we reach edge or different piece/empty
            while i < (self.height as usize) && j < (self.width as usize) 
                && self.spaces[i][j].is_same(white_trn) {
                connected += 1;
                i += y_offset;
                j += x_offset;
            }

            //try subtracting offset until end of string of pieces
            //need to be careful not to underflow our unsigned indexes
            if *y_offset <= (height as usize) && *x_offset <= (move_choice as usize) {
                i = (height as usize) - y_offset;
                j = (move_choice as usize) - x_offset;
                while self.spaces[i][j].is_same(white_trn) {
                    connected += 1;
                if *y_offset > (i as usize) || *x_offset > (j as usize) {
                    break;
                }
                    i -= y_offset;
                    j -= x_offset;
                }
                
            }
            if connected >= 4 {
                    return true;
            }
        }
        return false;
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
        Game { board: Board::new(7, 6), white_trn: true, trn_cnt: 0 }
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
                    1..=7 => x,
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
    fn do_turn(&mut self) -> Result<bool, &str>  {
        let move_choice = self.get_move();
        match self.board.make_move(move_choice, self.white_trn) {
            Ok(x) => {
                self.white_trn = !self.white_trn;
                self.trn_cnt += 1;
                Ok(x)
            },
            Err(s) => {
                println!("{}", s);
                Err("failed to do turn")
            },
        }
    }
}
fn main() {
    let mut game = Game::new();
    game.board.print();
    loop {
        match game.do_turn() {
            Ok(true) => {
                game.board.print();
                let winner = match game.white_trn {
                    true => "black",
                    false => "white",
                };
                println!("{} wins!", winner);
                break;
            }
            _ => (),
        }
        game.board.print();
    }
}

