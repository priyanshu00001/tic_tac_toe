use::std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode,KeyEventKind},
    terminal::{Clear, ClearType, size},
};

enum Player {
    X,
    O
}

enum Cell {
    Empty,
    Occupide(Player)
}

enum GameResult {
    Win(Player),
    Draw,
    Ongoing
}

struct GameState {
    board: [Cell; 9],
    curr_turn: Player,
    scores: (u8, u8, u8),
    msg: String,
    result: GameResult
}

impl GameState {
    fn gen_sprite(&self, emty: &[&str; 6], o: &[&str; 6], x: &[&str; 6]) -> String {
        let mut str_buff = String::new();
        str_buff.push_str("Scores\n------------\n\n");

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

    fn update_state(&mut self, num_key: &char) {
        let key = *num_key as u8;

        if key < 48 || key > 57 {
            self.msg = String::from("Press Num Keys 0..9\n\nInvalid Key Press!\n");
            return;
        }

        let key = (key - 48) as usize;

        if let GameResult::Ongoing = self.result {
            if key == 0 {
                self.reset_board();
                self.result = GameResult::Draw;
                self.msg = String::from("Press num Keys 1 or 2\n\n1:X\n2:O");

            } else if let Cell::Empty = self.board[key-1] {
                if let Player::X = self.curr_turn {
                    self.board[key-1] = Cell::Occupide(Player::X);
                    self.curr_turn = Player::O;

                } else {
                    self.board[key-1] = Cell::Occupide(Player::O);
                    self.curr_turn = Player::X;
                }

                let result = self.check_result(&(key-1));

                if let GameResult::Draw = result{
                    self.reset_board();
                    self.scores.2+=if self.scores.2<255 {1} else {0};
                    self.msg = String::from("👍 It's a DRAW! 👍\n\nPress num Keys 1 or 2\n\n1:X\n2:O");

                }else if let GameResult::Win(Player::X) = result {
                    self.reset_board();
                    self.scores.1 += if self.scores.1<255{1} else {0};
                    self.msg = String::from("🎉 X WON! 🎉\n\nPress num Keys 1 or 2\n\n1:X\n2:O");

                }else if let GameResult::Win(Player::O) = result {
                    self.reset_board();
                    self.scores.0 += if self.scores.0<255{1} else {0};
                    self.msg = String::from("🎉 O WON! 🎉\n\nPress num Keys 1 or 2\n\n1:X\n2:O");
                }else{
                    self.msg = String::from("Press Num Keys 0..9");
                }

                self.result=result;

            } else {
                self.msg = String::from("Press Num Keys 0..9\n\nAlready Occupied!");
            }

        } else if key == 1 || key == 2 {
            if key == 1 {
                self.curr_turn = Player::X;

            } else if key == 2 {
                self.curr_turn = Player::O;
            }

            self.result = GameResult::Ongoing;
            self.msg = String::from("Press num Keys 0..9")

        } else {
            self.msg = String::from("Press num Keys 1 or 2\n\n1:X\n2:O\n\nInvalid Key Press!\n");
        }
    }

    fn reset_board(&mut self) {
        for i in 0..9usize {
            self.board[i] = Cell::Empty;
        }
    }

    fn check_result(&self, pos:&usize) -> GameResult{
        if let Cell::Occupide(Player::O) = self.board[*pos]{
            match pos {
                0 => {
                    if let Cell::Occupide(Player::O) = self.board[1] && let Cell::Occupide(Player::O) = self.board[2]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[3] && let Cell::Occupide(Player::O) = self.board[6]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[4] && let Cell::Occupide(Player::O) = self.board[8]{
                        return GameResult::Win(Player::O);
                    }
                },
                1 => {
                    if let Cell::Occupide(Player::O) = self.board[0] && let Cell::Occupide(Player::O) = self.board[2]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[4] && let Cell::Occupide(Player::O) = self.board[7]{
                        return GameResult::Win(Player::O);
                    }
                },
                2 => {
                    if let Cell::Occupide(Player::O) = self.board[0] && let Cell::Occupide(Player::O) = self.board[1]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[5] && let Cell::Occupide(Player::O) = self.board[8]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[4] && let Cell::Occupide(Player::O) = self.board[6]{
                        return GameResult::Win(Player::O);
                    }
                },
                3 => {
                    if let Cell::Occupide(Player::O) = self.board[4] && let Cell::Occupide(Player::O) = self.board[5]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[0] && let Cell::Occupide(Player::O) = self.board[6]{
                        return GameResult::Win(Player::O);
                    }
                },
                4 => {
                    if let Cell::Occupide(Player::O) = self.board[3] && let Cell::Occupide(Player::O) = self.board[5]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[1] && let Cell::Occupide(Player::O) = self.board[7]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[0] && let Cell::Occupide(Player::O) = self.board[8]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[2] && let Cell::Occupide(Player::O) = self.board[6]{
                        return GameResult::Win(Player::O);
                    }
                },
                5 => {
                    if let Cell::Occupide(Player::O) = self.board[4] && let Cell::Occupide(Player::O) = self.board[3]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[2] && let Cell::Occupide(Player::O) = self.board[8]{
                        return GameResult::Win(Player::O);
                    }
                },
                6 => {
                    if let Cell::Occupide(Player::O) = self.board[7] && let Cell::Occupide(Player::O) = self.board[8]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[0] && let Cell::Occupide(Player::O) = self.board[3]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[4] && let Cell::Occupide(Player::O) = self.board[2]{
                        return GameResult::Win(Player::O);
                    }
                },
                7 => {
                    if let Cell::Occupide(Player::O) = self.board[6] && let Cell::Occupide(Player::O) = self.board[8]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[1] && let Cell::Occupide(Player::O) = self.board[4]{
                        return GameResult::Win(Player::O);
                    }
                },
                8 => {
                    if let Cell::Occupide(Player::O) = self.board[6] && let Cell::Occupide(Player::O) = self.board[7]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[2] && let Cell::Occupide(Player::O) = self.board[5]{
                        return GameResult::Win(Player::O);
                    }
                    if let Cell::Occupide(Player::O) = self.board[0] && let Cell::Occupide(Player::O) = self.board[4]{
                        return GameResult::Win(Player::O);
                    }
                },
                _=>{}
            }

        }else {
            match pos {
                0 => {
                    if let Cell::Occupide(Player::X) = self.board[1] && let Cell::Occupide(Player::X) = self.board[2]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[3] && let Cell::Occupide(Player::X) = self.board[6]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[4] && let Cell::Occupide(Player::X) = self.board[8]{
                        return GameResult::Win(Player::X);
                    }
                },
                1 => {
                    if let Cell::Occupide(Player::X) = self.board[0] && let Cell::Occupide(Player::X) = self.board[2]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[4] && let Cell::Occupide(Player::X) = self.board[7]{
                        return GameResult::Win(Player::X);
                    }
                },
                2 => {
                    if let Cell::Occupide(Player::X) = self.board[0] && let Cell::Occupide(Player::X) = self.board[1]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[5] && let Cell::Occupide(Player::X) = self.board[8]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[4] && let Cell::Occupide(Player::X) = self.board[6]{
                        return GameResult::Win(Player::X);
                    }
                },
                3 => {
                    if let Cell::Occupide(Player::X) = self.board[4] && let Cell::Occupide(Player::X) = self.board[5]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[0] && let Cell::Occupide(Player::X) = self.board[6]{
                        return GameResult::Win(Player::X);
                    }
                },
                4 => {
                    if let Cell::Occupide(Player::X) = self.board[3] && let Cell::Occupide(Player::X) = self.board[5]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[1] && let Cell::Occupide(Player::X) = self.board[7]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[0] && let Cell::Occupide(Player::X) = self.board[8]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[2] && let Cell::Occupide(Player::X) = self.board[6]{
                        return GameResult::Win(Player::X);
                    }
                },
                5 => {
                    if let Cell::Occupide(Player::X) = self.board[4] && let Cell::Occupide(Player::X) = self.board[3]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[2] && let Cell::Occupide(Player::X) = self.board[8]{
                        return GameResult::Win(Player::X);
                    }
                },
                6 => {
                    if let Cell::Occupide(Player::X) = self.board[7] && let Cell::Occupide(Player::X) = self.board[8]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[0] && let Cell::Occupide(Player::X) = self.board[3]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[4] && let Cell::Occupide(Player::X) = self.board[2]{
                        return GameResult::Win(Player::X);
                    }
                },
                7 => {
                    if let Cell::Occupide(Player::X) = self.board[6] && let Cell::Occupide(Player::X) = self.board[8]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[1] && let Cell::Occupide(Player::X) = self.board[4]{
                        return GameResult::Win(Player::X);
                    }
                },
                8 => {
                    if let Cell::Occupide(Player::X) = self.board[6] && let Cell::Occupide(Player::X) = self.board[7]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[2] && let Cell::Occupide(Player::X) = self.board[5]{
                        return GameResult::Win(Player::X);
                    }
                    if let Cell::Occupide(Player::X) = self.board[0] && let Cell::Occupide(Player::X) = self.board[4]{
                        return GameResult::Win(Player::X);
                    }
                }
                _=>{}
            }
        }
        
        for cell in &self.board{
            if let &Cell::Empty = cell{
                return GameResult::Ongoing;
            }
        }

        GameResult::Draw
    }
}


fn render(sprite:&String, term_col:u16, term_row:u16){
    let mut string_buff=String::new();
    let mut row_count=0u16;
 
    let initial_spaces=" ".repeat(if term_col>=30 {((term_col-30)/2) as usize} else {0});    
    
    for line in sprite.lines(){
        row_count+=1;
        string_buff.push_str(&initial_spaces);
        let ext_epaces=" ".repeat((31usize - line.chars().count())/2);
        string_buff.push_str(&ext_epaces);
        string_buff.push_str(line);
        string_buff.push('\n');
    }
    string_buff.pop().unwrap();

    let skip_rows= if term_row>row_count {(term_row-row_count)/2}else{0};
    crossterm::execute!(stdout(), cursor::MoveTo(0, skip_rows)).unwrap();

    clear_screen();
    print!("{string_buff}");
    stdout().flush().unwrap();
    
    crossterm::execute!(stdout(), cursor::Hide).unwrap();
}

fn clear_screen() {
    crossterm::execute!(
        stdout(),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        // cursor::MoveTo(0, 0)
    )
    .unwrap();
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
        msg: String::from("Press num Keys 1 or 2\n\n1:X\n2:O"),
        result: GameResult::Draw,
    };

    let sz = size().unwrap();
    let sprite = game.gen_sprite(&EMPTY_CELL, &O_CELL, &X_CELL);
    render(&sprite, sz.0, sz.1);

    loop {
        match event::read().unwrap() {
            Event::Resize(new_cols, new_rows) => {
                let sprite = game.gen_sprite(&EMPTY_CELL, &O_CELL, &X_CELL);
                render(&sprite, new_cols, new_rows);
            }
            Event::Key(event) => {
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                if event.code == KeyCode::Esc {
                    crossterm::execute!(stdout(), cursor::Show).unwrap();
                    break;
                }
                
                if let KeyCode::Char(c) = event.code{
                    let s=size().unwrap();
                    if s.0>=30{
                        game.update_state(&c);
                        let sprite = game.gen_sprite(&EMPTY_CELL, &O_CELL, &X_CELL);
                        render(&sprite, s.0, s.1);
                    }else{
                        let st = "Small Terminal".to_string();
                        render(&st, s.0, s.1);
                    }
                }
            }
            _ => {}
        }
    }
}
