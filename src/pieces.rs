use macroquad::prelude::*;

use crate::{RAYWHITE, SQUARE};

#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    None,
    Pawn(Data),
    Knight(Data),
    Bishop(Data),
    Rook(Data),
    Queen(Data),
    King(Data),
}
impl Piece {
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

    pub fn deselect_every_piece(pieces: &mut Vec<Vec<Piece>>) {
        for j in 0..8 {
            for i in 0..8 {
                if pieces[j][i] == Piece::None {
                    continue;
                }
                pieces[j][i] = Piece::change_value(
                    &pieces[j][i],
                    Data {
                        tex: Piece::get_texture(&pieces[j][i]),
                        selected: false,
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn make_move(pieces: &mut Vec<Vec<Piece>>, index: (usize, usize), m: (usize, usize)) {
        pieces[m.0][m.1] = pieces[index.0][index.1].clone();
        pieces[index.0][index.1] = Piece::None;

        pieces[m.0][m.1] = Piece::change_value(&pieces[m.0][m.1], Data {
            tex: Piece::get_texture(&pieces[m.0][m.1]),
            ..Default::default()
        });
    }
}

pub fn check_for_move(pieces: &mut Vec<Vec<Piece>>) {
    if any_piece_selected(pieces) {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let mut index = (0, 0);

        for j in 0..8 {
            for i in 0..8 {
                if Piece::get_moves(&pieces[j][i]).len() > 0 {
                    moves = Piece::get_moves(&pieces[j][i]);
                    index = (j, i);
                    break;
                }
            }
        }
        if moves.len() > 0 {
            for j in 0..8 {
                for i in 0..8 {
                    if piece_clicked(i, j) {
                        for m in &moves {
                            if (j, i) == *m {
                                Piece::make_move(pieces, index, *m);
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    for j in 0..8 {
        for i in 0..8 {
            if pieces[j][i] == Piece::None {
                continue;
            }

            if piece_clicked(i, j) {
                Piece::deselect_every_piece(pieces);

                pieces[j][i] = Piece::change_value(
                    &pieces[j][i],
                    Data {
                        tex: Piece::get_texture(&pieces[j][i]),
                        selected: true,
                        moves: calculate_moves(&pieces[j][i], j, i),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn any_piece_selected(pieces: &mut Vec<Vec<Piece>>) -> bool {
    for j in 0..8 {
        for i in 0..8 {
            if Piece::get_if_selected(&pieces[j][i]) {
                return true;
            }
        }
    }
    false
}

fn calculate_moves(piece: &Piece, j: usize, i: usize) -> Vec<(usize, usize)> {
    match piece {
        Piece::Pawn(_) => {
            if j == 1 {
                vec![(j - 1, i), (j - 2, i)]
            } else {
                vec![(j - 1, i)]
            }
        }
        Piece::Knight(_) => {
            vec![(j - 2, i + 1), (j - 2, i - 1)]
        }
        _ => Vec::new(),
    }
}

fn piece_clicked(x: usize, y: usize) -> bool {
    let x = x as f32;
    let y = y as f32;

    return is_mouse_button_pressed(MouseButton::Left)
        && mouse_position().0 > x * SQUARE
        && mouse_position().0 < x * SQUARE + SQUARE
        && mouse_position().1 > y * SQUARE
        && mouse_position().1 < y * SQUARE + SQUARE;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    tex: Texture2D,
    selected: bool,
    color: Color,
    moves: Vec<(usize, usize)>,
}
impl Data {
    pub fn new(tex: Texture2D) -> Self {
        Self {
            tex: tex,
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
        }
    }
}
impl Default for Data {
    fn default() -> Self {
        Self {
            tex: Texture2D::empty(),
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
        }
    }
}

/*#[derive(Debug, PartialEq)]
enum DataMember {
    Path,
    Clicked,
    Color,
}*/
