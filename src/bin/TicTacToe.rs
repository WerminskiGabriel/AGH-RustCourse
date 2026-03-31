use std::io::{self, Write};

fn main() {
    fn board_print(board: [[char; 3]; 3]) {
        for i in 0..3 {
            print!(" {} | {} | {} \n", board[i][0], board[i][1], board[i][2]);
            if i != 2 {
                println!(" ---------");
            }
        }
    }
    fn input() -> usize {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to readline");
        line.trim().parse().expect("input is not int")
    }
    fn board_parse(board: &mut [[char; 3]; 3]) {
        let mut idx = 1;
        for i in 0..3 {
            for j in 0..3 {
                board[i][j] = (b'0' + idx) as char;
                idx += 1;
            }
        }
    }

    fn board_move(board: &mut [[char; 3]; 3], idx: usize, mark: char) {
        let idx = idx - 1;
        let i = idx / 3;
        let j = idx % 3;
        board[i][j] = mark;
    }

    fn win_check(board: &mut [[char; 3]; 3], mark_A: char) -> (bool, bool) {
        let mut A_won = false;
        let mut B_won = false;

        for i in 0..3 {
            if (board[i][0] == board[i][1] && board[i][1] == board[i][2]) {
                if board[i][0] == mark_A {
                    A_won = true;
                    break;
                } else {
                    B_won = true;
                    break;
                }
            } else if (board[0][i] == board[1][i] && board[1][i] == board[2][i]) {
                if board[0][i] == mark_A {
                    A_won = true;
                    break;
                } else {
                    B_won = true;
                    break;
                }
            }
        }
        if A_won || B_won {
            return (A_won, B_won);
        } else if (board[0][0] == board[1][1] && board[1][1] == board[2][2])
            || (board[2][0] == board[1][1] && board[1][1] == board[0][2])
        {
            if board[1][1] == mark_A {
                A_won = true;
            } else {
                B_won = true;
            }
            return (A_won, B_won);
        }

        return (false, false);
    }

    let mut board = [['0'; 3]; 3];
    board_parse(&mut board);
    let mut idx: usize = 0;

    let start_mark = 'O';
    let scnd_mark = 'X';
    loop {
        let current_mark = if idx % 2 == 0 { start_mark } else { scnd_mark };
        print!(
            "Player {} ({}), Your turn (enter cell number from 1 to 9): ",
            { idx % 2 },
            { current_mark }
        );
        io::stdout().flush();
        let cell_idx = input();
        board_move(&mut board, cell_idx, current_mark);
        board_print(board);

        let mut A_won;
        let mut B_won;
        (A_won, B_won) = win_check(&mut board, start_mark);
        if A_won || B_won {
            println!(
                "Player {} won!!!!!",
                (if A_won { start_mark } else { scnd_mark })
            );
            break;
        }
        idx += 1;
    }
}
