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

// convention [row, col]
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Position(u32, u32);

pub struct Square {
    pub pos: Position,
    pub is_mine: bool,
    pub adjacent_mines: u32,
    pub state: SquareState,
}

pub struct MineSweeper {
    width: u32,
    height: u32,
    num_mines: u32,
    num_flagged: u32,
    rng: ThreadRng,
    mines_index: Vec<usize>,
    map: HashMap<Position, Square>,
}

impl MineSweeper {
    pub fn new(width: u32, height: u32, num_mines: u32) -> MineSweeper {
        let mut rng = thread_rng();

        let mut game = MineSweeper {
            width,
            height,
            num_mines,
            num_flagged: 0,
            mines_index: sample_indices(&mut rng, (height * width) as usize, num_mines as usize),
            rng,
            map: HashMap::new(),
        };

        game.populate_board();

        game
    }

    pub fn reset(&mut self) {
        self.num_flagged = 0;
        self.mines_index = sample_indices(
            &mut self.rng,
            (self.height * self.width) as usize,
            self.num_mines as usize,
        );
        self.map.clear();
        self.populate_board();
    }

    fn populate_board(&mut self) {
        for index in &self.mines_index {
            let i = *index as u32 / self.height;
            let j = *index as u32 % self.width;
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

        for i in 0..self.height {
            for j in 0..self.width {
                let neighbors =
                    MineSweeper::get_neighbor_coords(&Position(i, j), self.width, self.height);
                let adjacent_mines = neighbors
                    .iter()
                    .filter(|&x| {
                        self.mines_index
                            .contains(&((x.0 * self.width + x.1) as usize))
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

    fn get_neighbor_coords(curr_pos: &Position, width: u32, height: u32) -> HashSet<Position> {
        let mut neighbors: HashSet<Position> = HashSet::new();

        if curr_pos.0 > 0 {
            neighbors.insert(Position(curr_pos.0 - 1, curr_pos.1));

            if curr_pos.1 > 0 {
                neighbors.insert(Position(curr_pos.0 - 1, curr_pos.1 - 1));
            }

            if curr_pos.1 < width - 1 {
                neighbors.insert(Position(curr_pos.0 - 1, curr_pos.1 + 1));
            }
        }

        if curr_pos.1 > 0 {
            neighbors.insert(Position(curr_pos.0, curr_pos.1 - 1));
        }

        if curr_pos.0 < height - 1 {
            neighbors.insert(Position(curr_pos.0 + 1, curr_pos.1));

            if curr_pos.1 > 0 {
                neighbors.insert(Position(curr_pos.0 + 1, curr_pos.1 - 1));
            }

            if curr_pos.1 < width - 1 {
                neighbors.insert(Position(curr_pos.0 + 1, curr_pos.1 + 1));
            }
        }

        if curr_pos.1 < width - 1 {
            neighbors.insert(Position(curr_pos.0, curr_pos.1 + 1));
        }

        neighbors
    }

    pub fn show(&self) {
        let mut table = Table::new();

        for i in 0..self.height {
            let mut row: Vec<Cell> = Vec::new();
            for j in 0..self.width {
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

    pub fn check_game_won(&self) -> bool {
        if self.num_flagged < self.num_mines {
            false
        } else {
            self.map
                .values()
                .all(|x| !x.is_mine || x.state == SquareState::Flagged)
        }
    }

    pub fn check_game_lost(&self) -> bool {
        self.map
            .values()
            .any(|x| x.is_mine && x.state == SquareState::Revealed)
    }

    pub fn toggle_flag_square(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);

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
                    for p in MineSweeper::get_neighbor_coords(&pos, self.width, self.height) {
                        candidates.push_back(p);
                    }
                }
            }
        }

        all_reveal
    }

    pub fn reveal_square(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);

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
