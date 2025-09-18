fn check_board_matches_game(board: &str, game: &theseus::Game) {
    for (i, row) in board.lines().enumerate() {
        for (j, cell) in row.chars().enumerate() {
            match cell {
                'X' => assert!(
                    game.is_wall(i, j),
                    "Expected wall but did not find one at ({}, {}) while checking board {}",
                    i,
                    j,
                    board
                ),
                ' ' => assert!(
                    game.is_empty(i, j),
                    "Expected empty but did not find one at ({}, {}) while checking board {}",
                    i,
                    j,
                    board
                ),
                'G' => assert!(
                    game.is_goal(i, j),
                    "Expected goal but did not find one at ({}, {}) while checking board {}",
                    i,
                    j,
                    board
                ),
                'T' => assert!(
                    game.is_theseus(i, j),
                    "Expected Theseus but did not find one at ({}, {}) while checking board {}",
                    i,
                    j,
                    board
                ),
                'M' => assert!(
                    game.is_minotaur(i, j),
                    "Expected Minotaur but did not find one at ({}, {}) while checking board {}",
                    i,
                    j,
                    board
                ),
                _ => panic!("Unknown character in board: {}", cell),
            }
        }
    }
}

fn check_board(board: &str) {
    let game = theseus::Game::from_board(board)
        .expect(&format!("Failed to create game from board {}", board));
    check_board_matches_game(board, &game);
}

#[test]
fn test_load_board_30() {
    let board0 = "XXXX\n\
                  XMTX\n\
                  X GX\n\
                  XXXX\n";
    let board1 = "XXXXXXXXX\n\
                  X  T  XXX\n\
                  X XXX XXX\n\
                  X   X  GX\n\
                  X XXX XXX\n\
                  X  M  XXX\n\
                  XXXXXXXXX\n";
    let board2 = "XXXXXXXXXXXXXXX\n\
                  X             X\n\
                  X             X\n\
                  XMX X         X\n\
                  X XXXT        X\n\
                  X         X X X\n\
                  X         XXX X\n\
                  X             X\n\
                  XXXX XXXXXXXXXX\n\
                  X  XGX         \n";

    check_board(board0);
    check_board(board1);
    check_board(board2);
}

fn check_theseus_move(start_board: &str, command: theseus::Command, finish_board: &str) {
    let mut game = theseus::Game::from_board(start_board)
        .expect(&format!("Failed to create game from board {}", start_board));

    check_board_matches_game(start_board, &game);
    game.theseus_move(command);
    check_board_matches_game(finish_board, &game);
}

#[test]
fn test_theseus_move_basic_10() {
    let start_board = "XXXXXX\n\
                        X    X\n\
                        XM T X\n\
                        XG   X\n\
                        XXXXXX\n";
    let command = theseus::Command::Right;
    let finish_board = "XXXXXX\n\
                        X    X\n\
                        XM  TX\n\
                        XG   X\n\
                        XXXXXX\n";
    check_theseus_move(start_board, command, finish_board);

    let start_board = "XXXXXX\n\
                        X    X\n\
                        XM T X\n\
                        XG   X\n\
                        XXXXXX\n";
    let command = theseus::Command::Left;
    let finish_board = "XXXXXX\n\
                        X    X\n\
                        XMT  X\n\
                        XG   X\n\
                        XXXXXX\n";
    check_theseus_move(start_board, command, finish_board);

    let start_board = "XXXXXX\n\
                        X    X\n\
                        XM T X\n\
                        XG   X\n\
                        XXXXXX\n";
    let command = theseus::Command::Up;
    let finish_board = "XXXXXX\n\
                        X  T X\n\
                        XM   X\n\
                        XG   X\n\
                        XXXXXX\n";
    check_theseus_move(start_board, command, finish_board);

    let start_board = "XXXXXX\n\
                        X    X\n\
                        XM T X\n\
                        XG   X\n\
                        XXXXXX\n";
    let command = theseus::Command::Down;
    let finish_board = "XXXXXX\n\
                        X    X\n\
                        XM   X\n\
                        XG T X\n\
                        XXXXXX\n";
    check_theseus_move(start_board, command, finish_board);

    let start_board = "XXXXXX\n\
                        X    X\n\
                        XM T X\n\
                        XG   X\n\
                        XXXXXX\n";
    let command = theseus::Command::Skip;
    check_theseus_move(start_board, command, start_board);
}

#[test]
fn test_theseus_move_disallowed_5() {
    let start_board = "XXXXXX\n\
                        X   XX\n\
                        XM XTX\n\
                        XG  XX\n\
                        XXXXXX\n";
    let commands = vec![
        theseus::Command::Up,
        theseus::Command::Down,
        theseus::Command::Left,
        theseus::Command::Right,
        theseus::Command::Skip,
    ];
    for command in commands {
        check_theseus_move(start_board, command, start_board);
    }
}

#[test]
fn test_minotaur_move_basic_10() {
    let boards = vec![
        "XXXXXXXXX\n\
         X     T X\n\
         X       X\n\
         X M    GX\n\
         XXXXXXXXX\n",
        "XXXXXXXXX\n\
         X     T X\n\
         X       X\n\
         X  M   GX\n\
         XXXXXXXXX\n",
        "XXXXXXXXX\n\
         X     T X\n\
         X       X\n\
         X   M  GX\n\
         XXXXXXXXX\n",
        "XXXXXXXXX\n\
         X     T X\n\
         X       X\n\
         X    M GX\n\
         XXXXXXXXX\n",
        "XXXXXXXXX\n\
         X     T X\n\
         X       X\n\
         X     MGX\n\
         XXXXXXXXX\n",
        "XXXXXXXXX\n\
         X     T X\n\
         X     M X\n\
         X      GX\n\
         XXXXXXXXX\n",
    ];
    let mut game = theseus::Game::from_board(boards[0])
        .expect(&format!("Failed to create game from board {}", boards[0]));
    check_board_matches_game(&boards[0], &game);
    for (start_board, finish_board) in boards.iter().zip(boards.iter().skip(1)) {
        game.minotaur_move();
        check_board_matches_game(finish_board, &game);
    }
}

#[test]
fn test_minotaur_move_disallowed_10() {
    {
        let board = "XXXXXXX\n\
                     X  T  X\n\
                     X  X  X\n\
                     X  M GX\n\
                     XXXXXXX\n";
        let mut game = theseus::Game::from_board(board)
            .expect(&format!("Failed to create game from board {}", board));
        check_board_matches_game(&board, &game);

        game.minotaur_move();
        check_board_matches_game(&board, &game);
    }
    {
        let start_board = "XXXXXXX\n\
                           X   T X\n\
                           X     X\n\
                           XG MX X\n\
                           XXXXXXX\n";
        let finish_board = "XXXXXXX\n\
                            X   T X\n\
                            X  M  X\n\
                            XG  X X\n\
                            XXXXXXX\n";
        let mut game = theseus::Game::from_board(&start_board)
            .expect(&format!("Failed to create game from board {}", start_board));
        check_board_matches_game(&start_board, &game);

        game.minotaur_move();
        check_board_matches_game(&finish_board, &game);
    }
}

#[test]
fn test_status_win_4() {
    let board = "XXXXXX\n\
                 X    X\n\
                 XM TGX\n\
                 X    X\n\
                 XXXXXX\n";
    let mut game = theseus::Game::from_board(board)
        .expect(&format!("Failed to create game from board {}", board));
    check_board_matches_game(board, &game);

    game.theseus_move(theseus::Command::Right);
    assert_eq!(game.status(), theseus::GameStatus::Win);
}

#[test]
fn test_status_lose_3() {
    let board = "XXXXXX\n\
                 X    X\n\
                 XMT GX\n\
                 X    X\n\
                 XXXXXX\n";
    let mut game = theseus::Game::from_board(board)
        .expect(&format!("Failed to create game from board {}", board));
    check_board_matches_game(board, &game);

    game.minotaur_move();
    assert_eq!(game.status(), theseus::GameStatus::Lose);
}

#[test]
fn test_status_continue_3() {
    let board = "XXXXXX\n\
                 X    X\n\
                 XM TGX\n\
                 X    X\n\
                 XXXXXX\n";
    let mut game = theseus::Game::from_board(board)
        .expect(&format!("Failed to create game from board {}", board));
    check_board_matches_game(board, &game);
    assert_eq!(game.status(), theseus::GameStatus::Continue);
    game.minotaur_move();
    assert_eq!(game.status(), theseus::GameStatus::Continue);
}
