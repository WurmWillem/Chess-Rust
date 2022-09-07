use macroquad::prelude::*;

use crate::{textures::get_textures, RAYWHITE, SCREENSIZE};

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
        /*let mut da = Data {..Default::default()};
        change.typ
        if mem == DataMember::Path {
            da = Data {
                //path: change,
                ..Default::default()
            }
        }*/

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

    pub fn get_if_clicked(piece: &Piece) -> bool {
        return match &piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.clicked,
            _ => false,
        };
    }
}

pub fn check_for_move(pieces: &mut Vec<Vec<Piece>>, textures: &Vec<Texture2D>) {
    for j in 0..8 {
        for i in 0..8 {
            if pieces[j][i] == Piece::None {
                continue;
            }

            if piece_clicked(i, j) {
                pieces[j][i] = Piece::change_value(
                    &pieces[j][i],
                    Data {
                        tex: Piece::get_texture(&pieces[j][i]),
                        color: RED,
                        clicked: true,
                        ..Default::default()
                    },
                );
                println!("changed");
            }
            if Piece::get_if_clicked(&pieces[j][i]) {
                pieces[j][i] = Piece::change_value(
                    &pieces[j][i],
                    Data {
                        tex: Piece::get_texture(&pieces[j][i]),
                        color: RED,
                        clicked: true,
                        ..Default::default()
                    },
                )
            }
        }
    }
}

fn piece_clicked(x: usize, y: usize) -> bool {
    let x = x as f32;
    let y = y as f32;
    let square = SCREENSIZE / 8.0;

    return is_mouse_button_pressed(MouseButton::Left)
        && mouse_position().0 > x * square
        && mouse_position().0 < x * square + square
        && mouse_position().1 > y * square
        && mouse_position().1 < y * square + square;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    tex: Texture2D,
    clicked: bool,
    color: Color,
}
impl Data {
    pub fn new(tex: Texture2D) -> Self {
        Self {
            tex: tex,
            clicked: false,
            color: RAYWHITE,
        }
    }
}
impl Default for Data {
    fn default() -> Self {
        Self {
            tex: Texture2D::empty(),
            clicked: false,
            color: RED,
        }
    }
}

/*#[derive(Debug, PartialEq)]
enum DataMember {
    Path,
    Clicked,
    Color,
}*/
