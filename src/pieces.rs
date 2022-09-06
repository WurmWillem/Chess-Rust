use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Piece {
    None,
    Pawn(String),
    Knight(String),
    Bishop(String),
    Rook(String),
    Queen(String),
    King(String),
}
impl Piece {
    pub fn get_path(piece: &Piece) -> String {
        match piece {
            Piece::None => "".to_string(),
            Piece::Pawn(str)
            | Piece::Knight(str)
            | Piece::Bishop(str)
            | Piece::Rook(str)
            | Piece::Queen(str)
            | Piece::King(str) => str.to_string(),
        }
    }
}

pub fn check_for_move(pieces: &mut Vec<Vec<Piece>>) {
    for j in 0..8 {
        for i in 0..8 {
            if pieces[j][i] == Piece::None {
                continue;
            }

            if is_mouse_button_pressed(MouseButton::Left) {}
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Data {
    
}
