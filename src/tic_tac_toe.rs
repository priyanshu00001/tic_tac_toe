use::std::io::{stdout, Write};
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
};

pub enum Player {
    X,
    O
}

pub enum Cell {
    Empty,
    Occupide(Player)
}

pub enum GameResult {
    Win(Player),
    Draw,
    Ongoing
}

pub struct GameState {
    pub board: [Cell; 9],
    pub curr_turn: Player,
    pub scores: (u8, u8, u8),
    pub msg: String,
    pub result: GameResult
}

impl GameState {
    pub fn gen_sprite(&self, emty: &[&str; 6], o: &[&str; 6], x: &[&str; 6]) -> String {
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

    pub fn update_state(&mut self, num_key: &char) {
        let key = *num_key as u8;

        if key < 48 || key > 57 {
            self.msg = format!("{}\n\nInvalid Key Press!\n",if let GameResult::Ongoing = self.result {"Press Num Keys 0..9"} else{"Press num Keys 1 or 2\n\n1:X\n2:O"});
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

pub fn render(sprite:&String, term_col:u16, term_row:u16){
    let mut string_buff=String::new();
    let mut row_count=0u16;
 
    let initial_spaces=" ".repeat(if term_col>=30 {((term_col-30)/2) as usize} else {0});
    let max_cols = if term_col>=30 {30usize} else {0};

    for line in sprite.lines(){
        row_count+=1;
        string_buff.push_str(&initial_spaces);
        let mut ext_epaces=String::from("");
        if max_cols == 30{
            ext_epaces=" ".repeat((max_cols - line.chars().count())/2);
        }
        string_buff.push_str(&ext_epaces);
        string_buff.push_str(line);
        string_buff.push('\n');
    }
    string_buff.pop().unwrap();

    let skip_rows= if term_row>row_count {(term_row-row_count)/2}else{0};

    clear_screen();
    crossterm::execute!(stdout(), cursor::MoveTo(0, skip_rows)).unwrap();

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
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}