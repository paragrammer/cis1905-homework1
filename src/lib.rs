use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Lose,
    Continue,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardError {
    InvalidCharacter(char),
    InvalidSize,
    NoMinotaur,
    NoTheseus,
    NoGoal,
    MultipleMinotaur,
    MultipleTheseus,
    MultipleGoal,
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            BoardError::InvalidSize => write!(f, "Invalid size"),
            BoardError::NoMinotaur => write!(f, "No minotaur"),
            BoardError::NoTheseus => write!(f, "No theseus"),
            BoardError::NoGoal => write!(f, "No goal"),
            BoardError::MultipleMinotaur => write!(f, "Multiple minotaur"),
            BoardError::MultipleTheseus => write!(f, "Multiple theseus"),
            BoardError::MultipleGoal => write!(f, "Multiple goal"),
        }
    }
}
impl Error for BoardError {}

#[derive(Clone)]
pub struct Grid {
    // TODO: Implement the Grid struct
}
impl Grid {
    // TODO: Implement the Grid struct
}

#[derive(Clone)]
pub struct Game {
    grid: Grid,
    // TODO: Implement the Game struct
}

impl Game {
    // TODO: replace the function body with your implementation
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        return Ok(Game {
            grid: Grid {},
        });
    }

    // TODO
    pub fn show(&self) {
    }

    // TODO
    pub fn minotaur_move(&mut self) {
    }

    // TODO
    pub fn theseus_move(&mut self, command: Command) {
    }

    // TODO: replace the function body with your implementation
    pub fn status(&self) -> GameStatus {
        GameStatus::Continue
    }
}

impl Game {
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        false
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Move one tile up
    Up,
    /// Move one tile down
    Down,
    /// Move one tile left
    Left,
    /// Move one tile right
    Right,
    /// Don't move at all
    Skip,
}

//  To get a command from the user, you can use the following code:
//  ```
//  let line = stdin.lines().next().unwrap().unwrap();
//  ```
//  This will read a line from the user and store it in the `buffer` string.
//  
//  Unfortunately, since stdin is line-buffered, everytime you enter a command while playing the
//  game you will have to press "enter" afterwards to send a new line.
//
//  While using the arrow keys to take inputs would be natural, it can be difficult to handle arrow
//  keys in a way that works on all devices. Therefore, it's recommended that you either use "w",
//  "a", "s", and "d" to take input, or else the words "up", "down", "left", "right". You can take
//  input however you like, so long as you document it here in a comment and it is reasonable to
//  use as a player.
pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    // TODO: replace this loop with your code
    loop {}
}
