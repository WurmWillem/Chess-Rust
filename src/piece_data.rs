use macroquad::{texture::Texture2D, prelude::Color};

use crate::{state::Side, RAYWHITE, pieces::Piece};

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub tex: Texture2D,
    pub selected: bool,
    pub color: Color,
    pub moves: Vec<(usize, usize)>,
    pub side: Side,
}
impl Data {
    pub fn new(tex: Texture2D, side: Side) -> Self {
        Self {
            tex: tex,
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
            side: side,
        }
    }

    pub fn get_new(piece: &Piece) -> Self {
        Self {
            tex: Data::get_texture(piece),
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
            side: Data::get_side(piece),
        }
    }

    pub fn change_value(piece: &Piece, dat: Data) -> Piece {
        return match &piece {
            Piece::Pawn(_) => Piece::Pawn(dat),
            Piece::Knight(_) => Piece::Knight(dat),
            Piece::Bishop(_) => Piece::Bishop(dat),
            Piece::Rook(_) => Piece::Rook(dat),
            Piece::Queen(_) => Piece::Queen(dat),
            Piece::King(_) => Piece::King(dat),
            _ => Piece::None,
        };
    }

    pub fn get_texture(piece: &Piece) -> Texture2D {
        match piece {
            Piece::None => Texture2D::empty(),
            Piece::Pawn(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Rook(data)
            | Piece::Queen(data)
            | Piece::King(data) => data.tex.clone(),
        }
    }

    pub fn get_color(piece: &Piece) -> Color {
        match piece {
            Piece::None => RAYWHITE,
            Piece::Pawn(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Rook(data)
            | Piece::Queen(data)
            | Piece::King(data) => data.color.clone(),
        }
    }

    pub fn get_if_selected(piece: &Piece) -> bool {
        return match &piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.selected,
            _ => false,
        };
    }

    pub fn get_moves(piece: &Piece) -> Vec<(usize, usize)> {
        return match piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.moves.to_vec(),
            _ => Vec::new(),
        };
    }

    pub fn get_side(piece: &Piece) -> Side {
        return match piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.side.clone(),
            _ => Side::None,
        };
    }
}
impl Default for Data {
    fn default() -> Self {
        Self {
            tex: Texture2D::empty(),
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
            side: Side::None,
        }
    }
}