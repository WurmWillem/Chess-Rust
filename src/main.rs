use macroquad::prelude::*;

pub const RAYWHITE: Color = Color::new(0.96, 0.96, 0.96, 1.00);
pub const SCREENSIZE: f32 = 600.0;

mod pieces;
use pieces::Piece;

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new();

    loop {
        clear_background(BLACK);
        pieces::check_for_move(&mut board.pieces);
        board.draw().await;

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
    fn new() -> Self {
        let mut none = Vec::new();
        for _ in 0..8 {
            none.push(Piece::None);
        }

        let mut pieces = Vec::new();
        for _ in 0..8 {
            pieces.push(none.to_vec());
        }

        let white_pieces = vec![
            Piece::Rook("images/wrook.png".to_string()),
            Piece::Knight("images/wknight.png".to_string()),
            Piece::Bishop("images/wbishop.png".to_string()),
            Piece::Queen("images/wqueen.png".to_string()),
            Piece::King("images/wking.png".to_string()),
            Piece::Bishop("images/wbishop.png".to_string()),
            Piece::Knight("images/wknight.png".to_string()),
            Piece::Rook("images/wrook.png".to_string()),
        ];

        let black_pieces = vec![
            Piece::Rook("images/brook.png".to_string()),
            Piece::Knight("images/bknight.png".to_string()),
            Piece::Bishop("images/bbishop.png".to_string()),
            Piece::Queen("images/bqueen.png".to_string()),
            Piece::King("images/bking.png".to_string()),
            Piece::Bishop("images/bbishop.png".to_string()),
            Piece::Knight("images/bknight.png".to_string()),
            Piece::Rook("images/brook.png".to_string()),
        ];

        for j in 0..8 {
            for i in 0..8 {
                if j == 2 || j == 3 || j == 4 || j == 5 {
                    pieces[j][i] = Piece::None;
                } else if j == 1 {
                    pieces[j][i] = Piece::Pawn("images/wpawn.png".to_string());
                } else if j == 6 {
                    pieces[j][i] = Piece::Pawn("images/bpawn.png".to_string());
                } else if j == 0 {
                    pieces[j] = white_pieces.to_vec();
                } else if j == 7 {
                    pieces[j] = black_pieces.to_vec();
                }
            }
        }
        Self { pieces }
    }

    async fn draw(&self) {
        let board = load_texture("images/board.png").await.unwrap();
        let board_params = self.get_param(SCREENSIZE, SCREENSIZE);

        draw_texture_ex(board, 0.0, 0.0, RAYWHITE, board_params);

        for j in 0..8 {
            for i in 0..8 {
                let path = Piece::get_path(&self.pieces[j][i]);
                if path == "".to_string() {
                    continue;
                }

                let texture = load_texture(&path).await.unwrap();
                let params = self.get_param(SCREENSIZE / 8.0 - 3.0, SCREENSIZE / 8.0 - 3.0);

                draw_texture_ex(
                    texture,
                    i as f32 * (SCREENSIZE / 8.0) + 1.0,
                    j as f32 * (SCREENSIZE / 8.0) + 3.0,
                    RAYWHITE,
                    params,
                );
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
