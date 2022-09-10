use macroquad::prelude::*;

pub async fn get_textures() -> Vec<Texture2D> {
    return vec![
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
        load_texture("images/board.png").await.unwrap(),
    ];
}
