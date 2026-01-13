enum Player {
    X,
    O,
}

enum Cell {
    Empty,
    Occupide(Player),
}

enum GameResult {
    Win(Player),
    Draw,
    Ongoing,
}

struct GameState {
    board: [Cell; 9],
    curr_turn: Player,
    scores: (u8, u8, u8),
    msg: String,
    result: GameResult,
}

impl GameState {
    fn gen_sprite(&self, emty: &[&str; 6], o: &[&str; 6], x: &[&str; 6]) -> String {
        let mut str_buff = String::new();
        str_buff.push_str("Scores\n--------\n\n");

        let xzero = "0".repeat(if self.scores.0 > 99 {
            0
        } else if self.scores.0 > 9 {
            1
        } else {
            2
        });
        let ozero = "0".repeat(if self.scores.1 > 99 {
            0
        } else if self.scores.1 > 9 {
            1
        } else {
            2
        });
        let dzero = "0".repeat(if self.scores.2 > 99 {
            0
        } else if self.scores.2 > 9 {
            1
        } else {
            2
        });

        str_buff.push_str(&format!(
            "O : {}{}   X : {}{}   Draw : {}{}\n\n\n",
            xzero, self.scores.0, ozero, self.scores.1, dzero, self.scores.2
        ));
        if let GameResult::Ongoing = self.result {
            for i in (0..9usize).step_by(3) {
                for j in 0..6usize {
                    match self.board[i] {
                        Cell::Empty => str_buff.push_str(emty[j]),
                        Cell::Occupide(Player::O) => str_buff.push_str(o[j]),
                        Cell::Occupide(Player::X) => str_buff.push_str(x[j]),
                    }
                    match self.board[i + 1] {
                        Cell::Empty => str_buff.push_str(emty[j]),
                        Cell::Occupide(Player::O) => str_buff.push_str(o[j]),
                        Cell::Occupide(Player::X) => str_buff.push_str(x[j]),
                    }
                    match self.board[i + 2] {
                        Cell::Empty => str_buff.push_str(emty[j]),
                        Cell::Occupide(Player::O) => str_buff.push_str(o[j]),
                        Cell::Occupide(Player::X) => str_buff.push_str(x[j]),
                    }
                    str_buff.push('\n');
                }
            }
            str_buff.push_str("\n\n");
            str_buff.push_str(&format!(
                "Current Turn : {}\n\n",
                if let Player::O = self.curr_turn {
                    'O'
                } else {
                    'X'
                }
            ));
        }
        str_buff.push_str(&self.msg);
        str_buff
    }

    fn update_state(&mut self, num_key: &u8) {}
}

fn main() {
    const EMPTY_CELL: [&str; 6] = [
        "▏▔▔▔▔▔▔▔▕",
        "▏       ▕",
        "▏       ▕",
        "▏       ▕",
        "▏       ▕",
        "▏▁▁▁▁▁▁▁▕",
    ];
    const O_CELL: [&str; 6] = [
        "▏▔▔▔▔▔▔▔▕",
        "▏  ⧸▔⧹  ▕",
        "▏ ⧸   ⧹ ▕",
        "▏ ⧹   ⧸ ▕",
        "▏  ⧹▁⧸  ▕",
        "▏▁▁▁▁▁▁▁▕",
    ];
    const X_CELL: [&str; 6] = [
        "▏▔▔▔▔▔▔▔▕",
        "▏ ⧹   ⧸ ▕",
        "▏  ⧹▁⧸  ▕",
        "▏  ⧸ ⧹  ▕",
        "▏ ⧸   ⧹ ▕",
        "▏▁▁▁▁▁▁▁▕",
    ];

    let mut game = GameState {
        board: [
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ],
        curr_turn: Player::X,
        scores: (0, 0, 0),
        msg: String::from("Player O won!\n\nChoose your current turn:\n\nPress 1 : X\nPress 2 : O"),
        result: GameResult::Win(Player::O),
    };

    // game.board[2] = Cell::Occupide(Player::O);
    // game.board[4] = Cell::Occupide(Player::X);

    let sprite = game.gen_sprite(&EMPTY_CELL, &O_CELL, &X_CELL);
    println!("{sprite}");
}
