use super::*;

#[test]
fn test_check_game_won() {
    let mut game = MineSweeper::new(9, 9, 10);

    game.num_flagged = game.num_mines;

    assert!(!game.check_game_won());

    for index in &game.mines_index {
        assert!(!game.check_game_won());
        let i = *index as u32 / game.rows;
        let j = *index as u32 % game.cols;
        game.map.get_mut(&Position(i, j)).unwrap().state = SquareState::Flagged;
    }

    assert!(game.check_game_won());
}

#[test]
fn test_check_game_lost() {
    let game = MineSweeper::new(9, 9, 10);

    assert!(!game.check_game_lost());
}

#[test]
fn test_update_game_state() {
    let mut game = MineSweeper::new(9, 9, 10);
    game.update_game_state();

    assert_eq!(game.state, GameState::Ongoing);

    for index in &game.mines_index.clone() {
        assert!(!game.check_game_won());
        let i = *index as u32 / game.rows;
        let j = *index as u32 % game.cols;
        game.toggle_flag_square(&Position(i, j));
    }

    game.update_game_state();

    assert_eq!(game.state, GameState::Won);

    game.reset();

    assert_eq!(game.state, GameState::Ongoing);

    let mine_index = game.mines_index[0];
    let rows = game.rows;
    let cols = game.cols;
    game.reveal_square(&Position(
        mine_index as u32 / rows,
        mine_index as u32 % cols,
    ));

    // due to first move behavior where mine is moved, need to reveal start position
    let start_index = game.start_index;
    game.reveal_square(&Position(
        start_index as u32 / rows,
        start_index as u32 % cols,
    ));

    game.update_game_state();

    assert_eq!(game.state, GameState::Lost);
}

#[test]
fn test_first_move() {
    let mut game = MineSweeper::new(9, 9, 10);
    let mine_index = game.mines_index[0];
    let rows = game.rows;
    let cols = game.cols;

    let start_pos = Position(mine_index as u32 / rows, mine_index as u32 % cols);
    game.reveal_square(&start_pos);

    assert_eq!(game.state, GameState::Ongoing);

    assert!(!game.map[&start_pos].is_mine);

    let start_index = game.start_index;
    assert!(game.map[&Position(start_index as u32 / rows, start_index as u32 % cols,)].is_mine)
}

#[test]
fn test_get_neighbors() {
    let game = MineSweeper::new(9, 5, 10);

    let pos_1 = Position(0, 0);

    assert_eq!(
        hashset!{
        Position(1, 0),
        Position(0, 1),
        Position(1, 1)},
        MineSweeper::get_neighbor_coords(&pos_1, game.cols, game.rows)
    );

    let pos_2 = Position(2, 0);

    assert_eq!(
        hashset!{
        Position(2, 1),
        Position(1, 1),
        Position(1, 0),
        Position(3, 0),
        Position(3, 1)},
        MineSweeper::get_neighbor_coords(&pos_2, game.cols, game.rows)
    );

    let pos_3 = Position(4, 0);

    assert_eq!(
        hashset!{
        Position(4, 1),
        Position(3, 1),
        Position(3, 0)},
        MineSweeper::get_neighbor_coords(&pos_3, game.cols, game.rows)
    );

    let pos_4 = Position(4, 5);

    assert_eq!(
        hashset!{
        Position(4, 4),
        Position(4, 6),
        Position(3, 4),
        Position(3, 5),
        Position(3, 6)},
        MineSweeper::get_neighbor_coords(&pos_4, game.cols, game.rows)
    );

    let pos_5 = Position(4, 8);

    assert_eq!(
        hashset!{
        Position(4, 7),
        Position(3, 7),
        Position(3, 8)},
        MineSweeper::get_neighbor_coords(&pos_5, game.cols, game.rows)
    );

    let pos_6 = Position(2, 8);

    assert_eq!(
        hashset!{
        Position(2, 7),
        Position(3, 7),
        Position(1, 7),
        Position(1, 8),
        Position(3, 8)},
        MineSweeper::get_neighbor_coords(&pos_6, game.cols, game.rows)
    );

    let pos_7 = Position(0, 8);

    assert_eq!(
        hashset!{
        Position(0, 7),
        Position(1, 7),
        Position(1, 8)},
        MineSweeper::get_neighbor_coords(&pos_7, game.cols, game.rows)
    );

    let pos_8 = Position(0, 5);

    assert_eq!(
        hashset!{
        Position(0, 4),
        Position(0, 6),
        Position(1, 4),
        Position(1, 5),
        Position(1, 6)},
        MineSweeper::get_neighbor_coords(&pos_8, game.cols, game.rows)
    );

    let pos_9 = Position(2, 4);

    assert_eq!(
        hashset!{
        Position(2, 3),
        Position(2, 5),
        Position(1, 3),
        Position(1, 4),
        Position(1, 5),
        Position(3, 3),
        Position(3, 4),
        Position(3, 5)},
        MineSweeper::get_neighbor_coords(&pos_9, game.cols, game.rows)
    );
}

#[test]
fn test_adjacent_mines_num() {
    let mut game = MineSweeper {
        cols: 3,
        rows: 4,
        num_mines: 3,
        num_flagged: 0,
        rng: thread_rng(),
        mines_index: vec![0, 4, 8],
        map: HashMap::new(),
        first_move: true,
        timer: SystemTime::now(),
        elapsed: 0,
        start_index: 1,
        state: GameState::Ongoing,
    };

    game.populate_board();

    assert_eq!(game.map[&Position(0, 1)].adjacent_mines, 2);
    assert_eq!(game.map[&Position(0, 2)].adjacent_mines, 1);
    assert_eq!(game.map[&Position(1, 0)].adjacent_mines, 2);
    assert_eq!(game.map[&Position(1, 2)].adjacent_mines, 2);
    assert_eq!(game.map[&Position(2, 0)].adjacent_mines, 1);
    assert_eq!(game.map[&Position(2, 1)].adjacent_mines, 2);

    // checking integrity after first move mine move
    game.reveal_square(&Position(0, 0));

    assert_eq!(game.map[&Position(0, 0)].adjacent_mines, 2);
    assert_eq!(game.map[&Position(0, 2)].adjacent_mines, 2);
    assert_eq!(game.map[&Position(1, 0)].adjacent_mines, 2);
    assert_eq!(game.map[&Position(1, 2)].adjacent_mines, 3);
    assert_eq!(game.map[&Position(2, 0)].adjacent_mines, 1);
    assert_eq!(game.map[&Position(2, 1)].adjacent_mines, 2);
}

#[test]
fn test_toggle_flag() {
    let mut game = MineSweeper::new(9, 9, 10);

    assert_eq!(game.num_flagged, 0);
    assert_eq!(game.map[&Position(3, 4)].state, SquareState::Covered);
    assert_eq!(game.map[&Position(6, 7)].state, SquareState::Covered);
    game.toggle_flag_square(&Position(3, 4));
    game.toggle_flag_square(&Position(6, 7));
    assert_eq!(game.map[&Position(3, 4)].state, SquareState::Flagged);
    assert_eq!(game.map[&Position(6, 7)].state, SquareState::Flagged);
    assert_eq!(game.num_flagged, 2);
    game.toggle_flag_square(&Position(6, 7));
    assert_eq!(game.map[&Position(6, 7)].state, SquareState::Covered);
    assert_eq!(game.num_flagged, 1);
}

#[test]
fn test_toggle_flag_max() {
    let mut game = MineSweeper::new(9, 9, 5);
    assert_eq!(game.num_flagged, 0);

    for i in 0..5 {
        game.toggle_flag_square(&Position(i, 4));
    }

    assert_eq!(game.num_flagged, 5);

    game.toggle_flag_square(&Position(0, 3));

    assert_eq!(game.num_flagged, 5);
}

#[test]
fn test_reveal_square() {
    let mut game = MineSweeper {
        cols: 3,
        rows: 3,
        num_mines: 3,
        num_flagged: 0,
        rng: thread_rng(),
        mines_index: vec![0, 1, 5],
        map: HashMap::new(),
        first_move: true,
        timer: SystemTime::now(),
        elapsed: 0,
        start_index: 2,
        state: GameState::Ongoing,
    };

    game.populate_board();

    game.reveal_square(&Position(2, 0));

    assert_eq!(game.map[&Position(1, 0)].state, SquareState::Revealed);
    assert_eq!(game.map[&Position(1, 1)].state, SquareState::Revealed);
    assert_eq!(game.map[&Position(2, 0)].state, SquareState::Revealed);
    assert_eq!(game.map[&Position(2, 1)].state, SquareState::Revealed);

    assert_eq!(game.map[&Position(2, 2)].state, SquareState::Covered);
    assert_eq!(game.map[&Position(0, 1)].state, SquareState::Covered);
    assert_eq!(game.map[&Position(0, 0)].state, SquareState::Covered);
    assert_eq!(game.map[&Position(0, 2)].state, SquareState::Covered);
    assert_eq!(game.map[&Position(1, 2)].state, SquareState::Covered);
}

#[test]
fn test_reset() {
    let mut game = MineSweeper::new(9, 9, 10);

    game.toggle_flag_square(&Position(5, 5));

    game.reset();

    assert_eq!(game.get_square(5, 5).state, SquareState::Covered);
}
