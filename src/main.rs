use macroquad::prelude::*;

pub const RAYWHITE: Color = Color::new(0.96, 0.96, 0.96, 1.00);
pub const SCREENSIZE: f32 = 600.0;

mod pieces;
use pieces::{Data, Piece};

#[macroquad::main(window_conf)]
async fn main() {
    let board = Board::new().await;
    let mut i = 0;
    let board_ = load_texture("images/board.png").await.unwrap();
    loop {
        clear_background(BLACK);

        //pieces::check_for_move(&mut board.pieces);
        board.draw(&mut i, board_);

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
        let textures = vec![
            load_texture("images/wpawn.png").await.unwrap(),
            load_texture("images/wknight.png").await.unwrap(),
            load_texture("images/wbishop.png").await.unwrap(),
            load_texture("images/wrook.png").await.unwrap(),
            load_texture("images/wqueen.png").await.unwrap(),
            load_texture("images/wking.png").await.unwrap(),
            load_texture("images/bpawn.png").await.unwrap(),
            load_texture("images/bknight.png").await.unwrap(),
            load_texture("images/bbishop.png").await.unwrap(),
            load_texture("images/brook.png").await.unwrap(),
            load_texture("images/bqueen.png").await.unwrap(),
            load_texture("images/bking.png").await.unwrap(),
        ];

        let mut none = Vec::new();
        for _ in 0..8 {
            none.push(Piece::None);
        }

        let mut pieces = Vec::new();
        for _ in 0..8 {
            pieces.push(none.to_vec());
        }

        let white_pieces = vec![
            Piece::Rook(Data::new("images/wrook.png")),
            Piece::Knight(Data::new("images/wknight.png")),
            Piece::Bishop(Data::new("images/wbishop.png")),
            Piece::Queen(Data::new("images/wqueen.png")),
            Piece::King(Data::new("images/wking.png")),
            Piece::Bishop(Data::new("images/wbishop.png")),
            Piece::Knight(Data::new("images/wknight.png")),
            Piece::Rook(Data::new("images/wrook.png")),
        ];

        let black_pieces = vec![
            Piece::Rook(Data::new("images/brook.png")),
            Piece::Knight(Data::new("images/bknight.png")),
            Piece::Bishop(Data::new("images/bbishop.png")),
            Piece::Queen(Data::new("images/bqueen.png")),
            Piece::King(Data::new("images/bking.png")),
            Piece::Bishop(Data::new("images/bbishop.png")),
            Piece::Knight(Data::new("images/bknight.png")),
            Piece::Rook(Data::new("images/brook.png")),
        ];

        for j in 0..8 {
            for i in 0..8 {
                if j == 2 || j == 3 || j == 4 || j == 5 {
                    pieces[j][i] = Piece::None;
                } else if j == 6 {
                    pieces[j][i] = Piece::Pawn(Data::new("images/wpawn.png"));
                } else if j == 1 {
                    pieces[j][i] = Piece::Pawn(Data::new("images/bpawn.png"));
                } else if j == 7 {
                    pieces[j] = white_pieces.to_vec();
                } else if j == 0 {
                    pieces[j] = black_pieces.to_vec();
                }
            }
        }
        Self { pieces }
    }

    fn draw(&self, i: &mut i32, board: Texture2D) {
        //let board_ = load_texture("images/board.png").await.unwrap();

        let board_params = self.get_param(SCREENSIZE, SCREENSIZE);

        draw_texture_ex(board, 0.0, 0.0, RAYWHITE, board_params);

        for j in 0..8 {
            for i in 0..8 {
                let path = Piece::get_path(&self.pieces[j][i]);
                if path == "".to_string() {
                    continue;
                }

                /*if Piece::get_color(&self.pieces[j][i]) == RED {
                    println!("true");
                }
                else {
                    println!("false");
                }*/

                //let texture = load_texture(&path).await.unwrap();
                let params = self.get_param(SCREENSIZE / 8.0 - 3.0, SCREENSIZE / 8.0 - 3.0);

                /*draw_texture_ex(
                    texture,
                    i as f32 * (SCREENSIZE / 8.0) + 1.0,
                    j as f32 * (SCREENSIZE / 8.0) + 3.0,
                    Piece::get_color(&self.pieces[j][i]),
                    params,
                );*/
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
