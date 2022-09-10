use macroquad::prelude::*;

mod consts;
pub use consts::*;
mod pieces;
use pieces::{Data, Piece};
mod textures;
use textures::get_textures;

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new().await;

    loop {
        clear_background(BLACK);

        pieces::check_for_move(&mut board.pieces);
        board.draw();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_width: SCREENSIZE as i32,
        window_height: SCREENSIZE as i32,
        ..Default::default()
    }
}

struct Board {
    pub pieces: Vec<Vec<Piece>>,
}
impl Board {
    async fn new() -> Self {
        let textures = get_textures().await;
        let mut none = Vec::new();
        for _ in 0..8 {
            none.push(Piece::None);
        }

        let mut pieces = Vec::new();
        for _ in 0..8 {
            pieces.push(none.to_vec());
        }

        let white_pieces = vec![
            Piece::Rook(Data::new(textures[3])),
            Piece::Knight(Data::new(textures[1])),
            Piece::Bishop(Data::new(textures[2])),
            Piece::Queen(Data::new(textures[4])),
            Piece::King(Data::new(textures[5])),
            Piece::Bishop(Data::new(textures[2])),
            Piece::Knight(Data::new(textures[1])),
            Piece::Rook(Data::new(textures[3])),
        ];

        let black_pieces = vec![
            Piece::Rook(Data::new(textures[9])),
            Piece::Knight(Data::new(textures[7])),
            Piece::Bishop(Data::new(textures[8])),
            Piece::Queen(Data::new(textures[10])),
            Piece::King(Data::new(textures[11])),
            Piece::Bishop(Data::new(textures[8])),
            Piece::Knight(Data::new(textures[7])),
            Piece::Rook(Data::new(textures[9])),
        ];

        for j in 0..8 {
            for i in 0..8 {
                if j == 2 || j == 3 || j == 4 || j == 5 {
                    pieces[j][i] = Piece::None;
                } else if j == 6 {
                    pieces[j][i] = Piece::Pawn(Data::new(textures[0]));
                } else if j == 1 {
                    pieces[j][i] = Piece::Pawn(Data::new(textures[6]));
                } else if j == 7 {
                    pieces[j] = white_pieces.to_vec();
                } else if j == 0 {
                    pieces[j] = black_pieces.to_vec();
                }
            }
        }
        Self { pieces }
    }

    fn draw(&mut self) {
        self.draw_board();

        for j in 0..8 {
            for i in 0..8 {
                //self.pieces[j][i] = Piece::None;
                let tex = Piece::get_texture(&self.pieces[j][i]);
                if tex == Texture2D::empty() {
                    continue;
                }

                let params = self.get_param(SQUARE - 3.0, SQUARE - 3.0);

                draw_texture_ex(
                    tex,
                    i as f32 * SQUARE + 1.0,
                    j as f32 * SQUARE + 3.0,
                    Piece::get_color(&self.pieces[j][i]),
                    params,
                );
            }
        }
    }

    fn draw_board(&self) {
        for j in 0..8 {
            for i in 0..8 {
                if Piece::get_if_selected(&self.pieces[j][i]) {
                    draw_rectangle(
                        i as f32 * SQUARE,
                        j as f32 * SQUARE,
                        SQUARE,
                        SQUARE,
                        MY_YELLOW,
                    );
                    continue;
                }
                if (j + i) % 2 == 0 {
                    draw_rectangle(
                        i as f32 * SQUARE,
                        j as f32 * SQUARE,
                        SQUARE,
                        SQUARE,
                        MY_WHITE,
                    )
                } else {
                    draw_rectangle(
                        i as f32 * SQUARE,
                        j as f32 * SQUARE,
                        SQUARE,
                        SQUARE,
                        MY_GREEN,
                    )
                }
            }
        }
        for j in 0..8 {
            for i in 0..8 {
                let moves = Piece::get_moves(&self.pieces[j][i]);
                if moves.len() > 0 {
                    for m in moves {
                        draw_circle(
                            m.1 as f32 * SQUARE + SQUARE / 2.0,
                            m.0 as f32 * SQUARE + SQUARE / 2.0,
                            SQUARE / 6.0,
                            MY_GRAY,
                        );
                    }
                }
            }
        }
    }

    fn get_param(&self, x: f32, y: f32) -> DrawTextureParams {
        let params: DrawTextureParams = DrawTextureParams {
            dest_size: Some(vec2(x, y)),
            ..Default::default()
        };
        params
    }
}
