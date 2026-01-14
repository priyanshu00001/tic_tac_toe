use::std::io::{stdout};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode,KeyEventKind},
    terminal::{size},
};
mod tic_tac_toe;
use tic_tac_toe::{GameState,Cell,Player,GameResult,render};

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
                let s=size().unwrap();
                    if s.0>=30{
                        let sprite = game.gen_sprite(&EMPTY_CELL, &O_CELL, &X_CELL);
                        render(&sprite, new_cols, new_rows);

                    }else{
                        let st = "Small Terminal".to_string();
                        render(&st, s.0, s.1);
                    }
            },
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
