use crate::{piece_data::Data, pieces::*, SQUARE};
use macroquad::prelude::*;

#[derive(PartialEq, Debug)]
enum Turn {
    White,
    Black,
}
impl Turn {
    fn opposite(turn: &Turn) -> Turn {
        if *turn == Turn::White {
            Turn::Black
        } else if *turn == Turn::Black {
            Turn::White
        } else {
            panic!("tried to opposite None");
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    None,
    White,
    Black,
}
impl Side {
    fn opposite(side: &Side) -> Side {
        if *side == Side::White {
            Side::Black
        } else if *side == Side::Black {
            Side::White
        } else {
            panic!("tried to opposite None");
        }
    }
}

pub struct State {
    turn: Turn,
}
impl State {
    pub fn new() -> Self {
        Self { turn: Turn::White }
    }
    pub fn check_for_move(&mut self, pieces: &mut Vec<Vec<Piece>>) {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let mut index = (0, 0);

        for j in 0..8 {
            for i in 0..8 {
                if Data::get_moves(&pieces[j][i]).len() > 0 {
                    moves = Data::get_moves(&pieces[j][i]);
                    index = (j, i);
                    break;
                }
            }
        }
        for j in 0..8 {
            for i in 0..8 {
                if !square_clicked(i, j) {
                    continue;
                }

                let mut moved_piece = false;
                Piece::deselect_every_piece(pieces);

                if moves.len() > 0 {
                    let side_clicked = Data::get_side(&pieces[j][i]);
                    let side_original = Data::get_side(&pieces[index.0][index.1]);

                    for m in &moves {
                        if (j, i) == *m
                            && (side_clicked == Side::opposite(&side_original)
                                || side_clicked == Side::None
                                || side_original == Side::None)
                        {
                            Piece::make_move(pieces, index, *m);
                            self.turn = Turn::opposite(&self.turn);
                            moved_piece = true;
                            break;
                        }
                    }
                }

                let side = Data::get_side(&pieces[j][i]);
                if (side == Side::White && self.turn == Turn::Black)
                    || (side == Side::Black && self.turn == Turn::White)
                    || pieces[j][i] == Piece::None
                    || moved_piece
                {
                    continue;
                }

                pieces[j][i] = Data::change_value(
                    &pieces[j][i],
                    Data {
                        tex: Data::get_texture(&pieces[j][i]),
                        side: Data::get_side(&pieces[j][i]),
                        selected: true,
                        moves: Piece::calculate_moves(pieces, &pieces[j][i], j, i),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn square_clicked(x: usize, y: usize) -> bool {
    let x = x as f32;
    let y = y as f32;

    return is_mouse_button_pressed(MouseButton::Left)
        && mouse_position().0 > x * SQUARE
        && mouse_position().0 < x * SQUARE + SQUARE
        && mouse_position().1 > y * SQUARE
        && mouse_position().1 < y * SQUARE + SQUARE;
}
