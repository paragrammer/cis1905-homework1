use std::error::Error;
use std::fmt::Display;
use std::io::{self, BufRead};

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
            BoardError::InvalidSize => write!(f, "Invalid board size"),
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
    width: usize,
    height: usize,
    /// Underlying static map: 'X' = wall, ' ' = empty, 'G' = goal
    cells: Vec<char>,
}

impl Grid {
    pub fn new(width: usize, height: usize, cells: Vec<char>) -> Self {
        Self { width, height, cells }
    }
    #[inline]
    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
    }
    #[inline]
    fn idx(&self, row: usize, col: usize) -> Option<usize> {
        if self.in_bounds(row, col) {
            Some(row * self.width + col)
        } else {
            None
        }
    }
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        self.idx(row, col).map(|i| self.cells[i])
    }
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.get(row, col) == Some('X')
    }
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        self.get(row, col) == Some('G')
    }
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        matches!(self.get(row, col), Some(' ') )
    }
}

#[derive(Clone)]
pub struct Game {
    grid: Grid,
    theseus_row: usize,
    theseus_col: usize,
    minotaur_row: usize,
    minotaur_col: usize,
    goal_row: usize,
    goal_col: usize,
}

impl Game {
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        // Read lines, filter out empty trailing lines
        let lines: Vec<&str> = board.lines().collect();
        if lines.is_empty() {
            return Err(BoardError::InvalidSize);
        }
        let width = lines[0].chars().count();
        if width == 0 {
            return Err(BoardError::InvalidSize);
        }
        let height = lines.len();
        let mut cells: Vec<char> = Vec::with_capacity(width * height);

        // Track entities
        let mut t_pos: Option<(usize, usize)> = None;
        let mut m_pos: Option<(usize, usize)> = None;
        let mut g_pos: Option<(usize, usize)> = None;

        for (r, line) in lines.iter().enumerate() {
            if line.chars().count() != width {
                return Err(BoardError::InvalidSize);
            }
            for (c, ch) in line.chars().enumerate() {
                match ch {
                    'X' | ' ' | 'G' | 'T' | 'M' => {
                        // For the static grid, store 'X', ' ', or 'G'.
                        match ch {
                            'X' => cells.push('X'),
                            'G' => {
                                if g_pos.is_some() { return Err(BoardError::MultipleGoal); }
                                g_pos = Some((r, c));
                                cells.push('G');
                            }
                            'T' => {
                                if t_pos.is_some() { return Err(BoardError::MultipleTheseus); }
                                t_pos = Some((r, c));
                                cells.push(' ');
                            }
                            'M' => {
                                if m_pos.is_some() { return Err(BoardError::MultipleMinotaur); }
                                m_pos = Some((r, c));
                                cells.push(' ');
                            }
                            ' ' => cells.push(' '),
                            _ => unreachable!(),
                        }
                    }
                    other => return Err(BoardError::InvalidCharacter(other)),
                }
            }
        }

        let (tr, tc) = t_pos.ok_or(BoardError::NoTheseus)?;
        let (mr, mc) = m_pos.ok_or(BoardError::NoMinotaur)?;
        let (gr, gc) = g_pos.ok_or(BoardError::NoGoal)?;

        let grid = Grid::new(width, height, cells);

        Ok(Game {
            grid,
            theseus_row: tr,
            theseus_col: tc,
            minotaur_row: mr,
            minotaur_col: mc,
            goal_row: gr,
            goal_col: gc,
        })
    }

    pub fn show(&self) {
        for r in 0..self.grid.height {
            let mut line = String::with_capacity(self.grid.width);
            for c in 0..self.grid.width {
                if self.theseus_row == r && self.theseus_col == c {
                    line.push('T');
                } else if self.minotaur_row == r && self.minotaur_col == c {
                    line.push('M');
                } else if self.grid.is_wall(r, c) {
                    // Draw a block for walls
                    line.push('█');
                } else if self.grid.is_goal(r, c) {
                    line.push('G');
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line);
        }
    }

    pub fn minotaur_move(&mut self) {
        // Helper to test if move to (r,c) is valid (within bounds and not a wall)
        let try_move = |r: isize, c: isize| -> Option<(usize, usize)> {
            if r < 0 || c < 0 { return None; }
            let (r, c) = (r as usize, c as usize);
            if self.grid.in_bounds(r, c) && !self.grid.is_wall(r, c) {
                Some((r, c))
            } else {
                None
            }
        };

        let tx = self.theseus_col as isize;
        let ty = self.theseus_row as isize;
        let mx = self.minotaur_col as isize;
        let my = self.minotaur_row as isize;

        // 1) Try horizontal move that decreases |tx - mx|
        if tx < mx {
            if let Some((nr, nc)) = try_move(my, mx - 1) {
                self.minotaur_row = nr;
                self.minotaur_col = nc;
                return;
            }
        } else if tx > mx {
            if let Some((nr, nc)) = try_move(my, mx + 1) {
                self.minotaur_row = nr;
                self.minotaur_col = nc;
                return;
            }
        }

        // 2) Otherwise, try vertical move that decreases |ty - my|
        if ty < my {
            if let Some((nr, nc)) = try_move(my - 1, mx) {
                self.minotaur_row = nr;
                self.minotaur_col = nc;
                return;
            }
        } else if ty > my {
            if let Some((nr, nc)) = try_move(my + 1, mx) {
                self.minotaur_row = nr;
                self.minotaur_col = nc;
                return;
            }
        }
        // 3) Else: don't move
    }

    pub fn theseus_move(&mut self, command: Command) {
        let (dr, dc) = match command {
            Command::Up => (-1, 0),
            Command::Down => (1, 0),
            Command::Left => (0, -1),
            Command::Right => (0, 1),
            Command::Skip => (0, 0),
        };

        let new_r = self.theseus_row as isize + dr;
        let new_c = self.theseus_col as isize + dc;
        if new_r < 0 || new_c < 0 {
            return;
        }
        let (nr, nc) = (new_r as usize, new_c as usize);
        if self.grid.in_bounds(nr, nc) && !self.grid.is_wall(nr, nc) {
            self.theseus_row = nr;
            self.theseus_col = nc;
        }
    }

    pub fn status(&self) -> GameStatus {
        if self.theseus_row == self.minotaur_row && self.theseus_col == self.minotaur_col {
            return GameStatus::Lose;
        }
        if self.theseus_row == self.goal_row && self.theseus_col == self.goal_col {
            return GameStatus::Win;
        }
        GameStatus::Continue
    }
}

// Derived queries the autograder expects
impl Game {
    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        self.theseus_row == row && self.theseus_col == col
    }
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        self.minotaur_row == row && self.minotaur_col == col
    }
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.grid.is_wall(row, col)
    }
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        self.grid.is_goal(row, col)
    }
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        !self.is_theseus(row, col)
            && !self.is_minotaur(row, col)
            && !self.is_wall(row, col)
            && !self.is_goal(row, col)
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

pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    // Read one line. On EOF, return None (signals invalid/quit to caller loop).
    let mut reader = io::BufReader::new(stdin);
    let mut line = String::new();
    if reader.read_line(&mut line).ok()? == 0 {
        return None;
    }
    // Normalize
    let s = line.trim().to_lowercase();
    match s.as_str() {
        "w" | "up" => Some(Command::Up),
        "s" | "down" => Some(Command::Down),
        "a" | "left" => Some(Command::Left),
        "d" | "right" => Some(Command::Right),
        "" | "wait" | "skip" | "." => Some(Command::Skip),
        "q" | "quit" | "exit" => None,
        _ => None,
    }
}
