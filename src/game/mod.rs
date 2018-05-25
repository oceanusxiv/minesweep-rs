#[cfg(test)]
mod tests;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use rand::ThreadRng;
use rand::seq::sample_indices;
use rand::thread_rng;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SquareState {
    Covered,
    Flagged,
    Revealed,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameState {
    Ongoing,
    Won,
    Lost,
}

// convention [row, col]
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Position(pub u32, pub u32);

pub struct Square {
    pub pos: Position,
    pub is_mine: bool,
    pub adjacent_mines: u32,
    pub state: SquareState,
}

pub struct MineSweeper {
    pub cols: u32,
    pub rows: u32,
    pub num_mines: u32,
    num_flagged: u32,
    rng: ThreadRng,
    mines_index: Vec<usize>,
    map: HashMap<Position, Square>,
    pub state: GameState,
}

impl MineSweeper {
    pub fn new(cols: u32, rows: u32, num_mines: u32) -> MineSweeper {
        let mut rng = thread_rng();

        let mut game = MineSweeper {
            cols,
            rows,
            num_mines,
            num_flagged: 0,
            mines_index: sample_indices(&mut rng, (rows * cols) as usize, num_mines as usize),
            rng,
            map: HashMap::new(),
            state: GameState::Ongoing,
        };

        game.populate_board();

        game
    }

    pub fn reset(&mut self) {
        self.num_flagged = 0;
        self.mines_index = sample_indices(
            &mut self.rng,
            (self.rows * self.cols) as usize,
            self.num_mines as usize,
        );
        self.map.clear();
        self.populate_board();
        self.state = GameState::Ongoing;
    }

    fn populate_board(&mut self) {
        for index in &self.mines_index {
            let i = *index as u32 / self.rows;
            let j = *index as u32 % self.cols;
            self.map.insert(
                Position(i, j),
                Square {
                    pos: Position(i, j),
                    is_mine: true,
                    adjacent_mines: 0,
                    state: SquareState::Covered,
                },
            );
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                let neighbors =
                    MineSweeper::get_neighbor_coords(&Position(i, j), self.cols, self.rows);
                let adjacent_mines = neighbors
                    .iter()
                    .filter(|&x| {
                        self.mines_index
                            .contains(&((x.0 * self.cols + x.1) as usize))
                    })
                    .count() as u32;
                self.map.entry(Position(i, j)).or_insert(Square {
                    pos: Position(i, j),
                    is_mine: false,
                    adjacent_mines,
                    state: SquareState::Covered,
                });
            }
        }
    }

    fn get_neighbor_coords(curr_pos: &Position, cols: u32, rows: u32) -> HashSet<Position> {
        let mut neighbors: HashSet<Position> = HashSet::new();

        if curr_pos.0 > 0 {
            neighbors.insert(Position(curr_pos.0 - 1, curr_pos.1));

            if curr_pos.1 > 0 {
                neighbors.insert(Position(curr_pos.0 - 1, curr_pos.1 - 1));
            }

            if curr_pos.1 < cols - 1 {
                neighbors.insert(Position(curr_pos.0 - 1, curr_pos.1 + 1));
            }
        }

        if curr_pos.1 > 0 {
            neighbors.insert(Position(curr_pos.0, curr_pos.1 - 1));
        }

        if curr_pos.0 < rows - 1 {
            neighbors.insert(Position(curr_pos.0 + 1, curr_pos.1));

            if curr_pos.1 > 0 {
                neighbors.insert(Position(curr_pos.0 + 1, curr_pos.1 - 1));
            }

            if curr_pos.1 < cols - 1 {
                neighbors.insert(Position(curr_pos.0 + 1, curr_pos.1 + 1));
            }
        }

        if curr_pos.1 < cols - 1 {
            neighbors.insert(Position(curr_pos.0, curr_pos.1 + 1));
        }

        neighbors
    }

    pub fn show(&self) {
        let mut table = Table::new();

        for i in 0..self.rows {
            let mut row: Vec<Cell> = Vec::new();
            for j in 0..self.cols {
                let curr_square = &self.map[&Position(i, j)];
                if curr_square.is_mine {
                    row.push(Cell::new("M"));
                } else if curr_square.adjacent_mines > 0 {
                    row.push(Cell::new(&curr_square.adjacent_mines.to_string()));
                } else {
                    row.push(Cell::new(" "));
                }
            }
            table.add_row(Row::new(row));
        }

        table.printstd();
    }

    pub fn update_game_state(&mut self) {
        if self.check_game_won() {
            self.state = GameState::Won;
        } else if self.check_game_lost() {
            self.state = GameState::Lost;
        } else {
            self.state = GameState::Ongoing;
        }
    }

    fn check_game_won(&self) -> bool {
        if self.num_flagged < self.num_mines {
            false
        } else {
            self.map
                .values()
                .all(|x| !x.is_mine || x.state == SquareState::Flagged)
        }
    }

    fn check_game_lost(&self) -> bool {
        self.map
            .values()
            .any(|x| x.is_mine && x.state == SquareState::Revealed)
    }

    pub fn toggle_flag_square(&mut self, row: u32, col: u32) {
        assert!(row < self.rows);
        assert!(col < self.cols);

        if self.map[&Position(row, col)].state == SquareState::Flagged {
            self.map.get_mut(&Position(row, col)).unwrap().state = SquareState::Covered;
            self.num_flagged -= 1;
        } else {
            self.map.get_mut(&Position(row, col)).unwrap().state = SquareState::Flagged;
            self.num_flagged += 1;
        }
    }

    fn find_reveals(&self, row: u32, col: u32) -> HashSet<Position> {
        let mut all_reveal: HashSet<Position> = HashSet::new();
        let mut candidates: VecDeque<Position> = VecDeque::new();
        candidates.push_back(Position(row, col));
        let mut visited: HashSet<Position> = HashSet::new();

        while !candidates.is_empty() {
            let pos = candidates.pop_front().unwrap();
            if !visited.contains(&pos) {
                visited.insert(pos);
                all_reveal.insert(pos);

                let curr_square = &self.map[&Position(row, col)];

                if !curr_square.is_mine && curr_square.adjacent_mines == 0 {
                    for p in MineSweeper::get_neighbor_coords(&pos, self.cols, self.rows) {
                        candidates.push_back(p);
                    }
                }
            }
        }

        all_reveal
    }

    pub fn reveal_square(&mut self, row: u32, col: u32) {
        assert!(row < self.rows);
        assert!(col < self.cols);

        if self.map[&Position(row, col)].state == SquareState::Covered {
            let all_reveal = self.find_reveals(row, col);

            for pos in &all_reveal {
                self.map.get_mut(pos).unwrap().state = SquareState::Revealed;
            }
        }
    }

    pub fn get_square(&self, row: u32, col: u32) -> &Square {
        &self.map[&Position(row, col)]
    }
}
