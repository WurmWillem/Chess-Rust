use macroquad::prelude::*;

pub const RAYWHITE: Color = Color::new(0.96, 0.96, 0.96, 1.00);
pub const SCREENSIZE: f32 = 600.0;

mod pieces;
use pieces::{Data, Piece};
mod textures;
use textures::get_textures;

#[macroquad::main(window_conf)]
async fn main() {
    let board = Board::new().await;
    let mut i = 0;
    let textures = get_textures().await;
    loop {
        clear_background(BLACK);

        //pieces::check_for_move(&mut board.pieces);
        board.draw(&mut i, &textures);

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

    fn draw(&self, i: &mut i32, textures: &Vec<Texture2D>) {
        let board_params = self.get_param(SCREENSIZE, SCREENSIZE);
        draw_texture_ex(textures[12], 0.0, 0.0, RAYWHITE, board_params);

        for j in 0..8 {
            for i in 0..8 {
                let tex = Piece::get_texture(&self.pieces[j][i]);
                if tex == Texture2D::empty() {
                    continue;
                }

                let params = self.get_param(SCREENSIZE / 8.0 - 3.0, SCREENSIZE / 8.0 - 3.0);

                draw_texture_ex(
                    tex,
                    i as f32 * (SCREENSIZE / 8.0) + 1.0,
                    j as f32 * (SCREENSIZE / 8.0) + 3.0,
                    Piece::get_color(&self.pieces[j][i]),
                    params,
                );
            }
        }
        println!("{}", i);
        *i += 1;
    }

    fn get_param(&self, x: f32, y: f32) -> DrawTextureParams {
        let params: DrawTextureParams = DrawTextureParams {
            dest_size: Some(vec2(x, y)),
            ..Default::default()
        };
        params
    }
}
