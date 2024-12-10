use std::{fmt::Display, io::stdin, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => 'X',
                Player::O => 'O',
            }
        )
    }
}

struct Board {
    cells: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[None; 3]; 3],
            current_player: Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoardIndex {
    row: u8,
    col: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoardIndexParseError {
    WrongLength,
    RowOutOfRange,
    ColumnOutOfRange,
}

// A1, B2, C3...
impl FromStr for BoardIndex {
    type Err = BoardIndexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(BoardIndexParseError::WrongLength);
        }

        let s = s.as_bytes();
        if s[0] < b'A' || s[0] > b'C' {
            return Err(BoardIndexParseError::RowOutOfRange);
        }
        let row = s[0] - b'A';

        if s[1] < b'1' || s[1] > b'3' {
            return Err(BoardIndexParseError::ColumnOutOfRange);
        }
        let col = s[1] - b'1';

        Ok(Self { row, col })
    }
}

impl BoardIndex {
    #[allow(dead_code)]
    fn new(row: u8, col: u8) -> Self {
        if row >= 3 {
            panic!("Invalid row");
        }

        if col >= 3 {
            panic!("Invalid column");
        }

        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoardSetError {
    CellAlreadyOccupied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    InProgress,
    Win(Player),
    Draw,
}

impl Board {
    fn set(&mut self, coord: BoardIndex) -> Result<(), BoardSetError> {
        if self.cells[coord.row as usize][coord.col as usize].is_some() {
            return Err(BoardSetError::CellAlreadyOccupied);
        }

        self.cells[coord.row as usize][coord.col as usize] = Some(self.current_player);
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        Ok(())
    }

    fn check_win(&self) -> GameState {
        // Check rows
        for row in 0..3 {
            if let Some(player) = self.cells[row][0] {
                if self.cells[row][1] == Some(player) && self.cells[row][2] == Some(player) {
                    return GameState::Win(player);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if let Some(player) = self.cells[0][col] {
                if self.cells[1][col] == Some(player) && self.cells[2][col] == Some(player) {
                    return GameState::Win(player);
                }
            }
        }

        // Check diagonal (top-left to bottom-right)
        if let Some(player) = self.cells[0][0] {
            if self.cells[1][1] == Some(player) && self.cells[2][2] == Some(player) {
                return GameState::Win(player);
            }
        }

        // Check diagonal (top-right to bottom-left)
        if let Some(player) = self.cells[0][2] {
            if self.cells[1][1] == Some(player) && self.cells[2][0] == Some(player) {
                return GameState::Win(player);
            }
        }

        // Check for draw (all cells filled)
        if self
            .cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
        {
            return GameState::Draw;
        }

        GameState::InProgress
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells {
            write!(f, "|")?;
            for cell in row {
                write!(
                    f,
                    "{}",
                    match cell {
                        Some(Player::X) => 'X',
                        Some(Player::O) => 'O',
                        None => ' ',
                    }
                )?;
            }
            writeln!(f, "|")?;
            writeln!(f, "-----")?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::default();

    loop {
        println!("{board}");

        // Get the player's choice
        println!("Player {}, Enter a coordinate (A1, B2, C3...): ", board.current_player);
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let coord = buf.parse::<BoardIndex>();
        match coord {
            Ok(coord) => {
                // Update the board
                if board.set(coord).is_err() {
                    println!("Cell occupied");
                }
            }
            Err(e) => match e {
                BoardIndexParseError::WrongLength => println!("Invalid length"),
                BoardIndexParseError::RowOutOfRange => println!("Invalid row"),
                BoardIndexParseError::ColumnOutOfRange => println!("Invalid column"),
            },
        }

        // Check for a win
        match board.check_win() {
            GameState::Win(player) => {
                println!("{player} wins!");
                break;
            }
            GameState::Draw => {
                println!("Draw");
                break;
            }
            GameState::InProgress => {}
        }
    }
}
