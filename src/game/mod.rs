#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::SystemTime;

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
    first_move: bool,
    timer: SystemTime,
    elapsed: u64,
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
            first_move: true,
            timer: SystemTime::now(),
            elapsed: 0,
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
        self.first_move = true;
    }

    pub fn first_moved(&mut self) {
        if self.first_move {
            self.timer = SystemTime::now();
            self.first_move = false;
        }
    }

    pub fn game_time(&mut self) -> u64 {
        if self.first_move {
            0
        } else if self.state == GameState::Ongoing {
            self.elapsed = self.timer.elapsed().unwrap().as_secs();
            self.elapsed
        } else {
            self.elapsed
        }
    }

    fn populate_board(&mut self) {
        for index in &self.mines_index {
            let i = *index as u32 / self.cols;
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

    pub fn update_game_state(&mut self) {
        if self.check_game_won() {
            self.state = GameState::Won;
            self.map
                .iter_mut()
                .filter(|&(_, ref x)| x.state == SquareState::Covered)
                .for_each(|(_, ref mut x)| x.state = SquareState::Revealed);
        } else if self.check_game_lost() {
            self.state = GameState::Lost;
            self.map
                .iter_mut()
                .for_each(|(_, ref mut x)| x.state = SquareState::Revealed);
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

    pub fn toggle_flag_square(&mut self, curr_pos: &Position) {
        assert!(curr_pos.0 < self.rows);
        assert!(curr_pos.1 < self.cols);

        if self.map[curr_pos].state == SquareState::Flagged {
            self.map.get_mut(curr_pos).unwrap().state = SquareState::Covered;
            self.num_flagged -= 1;
        } else {
            self.map.get_mut(curr_pos).unwrap().state = SquareState::Flagged;
            self.num_flagged += 1;
        }
    }

    fn find_reveals(&self, curr_pos: &Position) -> HashSet<Position> {
        let curr_square = &self.map[curr_pos];
        if curr_square.is_mine || curr_square.adjacent_mines > 0 {
            return hashset!{curr_pos.clone()};
        }

        let mut all_reveal: HashSet<Position> = HashSet::new();
        let mut candidates: VecDeque<Position> = VecDeque::new();
        candidates.push_back(curr_pos.clone());
        let mut visited: HashSet<Position> = HashSet::new();

        while !candidates.is_empty() {
            let pos = candidates.pop_front().unwrap();
            if !visited.contains(&pos) {
                visited.insert(pos);

                let square = &self.map[&pos];

                if !square.is_mine && square.state != SquareState::Flagged {
                    all_reveal.insert(pos);
                }

                if !square.is_mine && square.state != SquareState::Flagged
                    && square.adjacent_mines == 0
                {
                    for p in MineSweeper::get_neighbor_coords(&pos, self.cols, self.rows) {
                        candidates.push_back(p);
                    }
                }
            }
        }

        all_reveal
    }

    pub fn reveal_square(&mut self, curr_pos: &Position) {
        assert!(curr_pos.0 < self.rows);
        assert!(curr_pos.1 < self.cols);

        if self.map[curr_pos].state == SquareState::Covered {
            let all_reveal = self.find_reveals(curr_pos);

            for pos in &all_reveal {
                self.map.get_mut(pos).unwrap().state = SquareState::Revealed;
            }
        }
    }

    pub fn get_square(&self, row: u32, col: u32) -> &Square {
        &self.map[&Position(row, col)]
    }

    pub fn get_flags_left(&self) -> u32 {
        self.num_mines - self.num_flagged
    }
}
