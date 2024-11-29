

use shogi::{Position, Square};
use crate::PieceButton;

pub struct Board<'a> {
    pub piece_buttons: [[PieceButton<'a>; 9]; 9], 
    pub active: [i32; 2],
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        let piece_buttons = std::array::from_fn(|_| {
            std::array::from_fn(|_| PieceButton::new())
        });

        Self {
            piece_buttons,
            active: [-1, -1],
        }
    }

    pub fn set_active(&mut self, rank: i32, file: i32) {
        if self.active == [rank, file] {
            self.active = [-1, -1];
        }
        else {
            self.active = [rank, file]
        }
    }

    pub fn update_board(&mut self, pos: &Position) {
        for rank in 0..9 {
            for file in 0..9 {
                let sq = Square::new(file, rank).unwrap();
                if let Some(piece) = pos.piece_at(sq) {
                    self.piece_buttons[rank as usize][file as usize] = PieceButton::new_piece(*piece);
                } 
                else {
                    self.piece_buttons[rank as usize][file as usize] = PieceButton::new();
                }
            }
        }
    }
}