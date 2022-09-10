use crate::piece_data::Data;

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
    pub fn deselect_every_piece(pieces: &mut Vec<Vec<Piece>>) {
        for j in 0..8 {
            for i in 0..8 {
                if pieces[j][i] == Piece::None {
                    continue;
                }
                pieces[j][i] = Data::change_value(&pieces[j][i], Data::get_new(&pieces[j][i]));
            }
        }
    }

    pub fn make_move(pieces: &mut Vec<Vec<Piece>>, index: (usize, usize), m: (usize, usize)) {
        pieces[m.0][m.1] = pieces[index.0][index.1].clone();
        pieces[index.0][index.1] = Piece::None;

        pieces[m.0][m.1] = Data::change_value(&pieces[m.0][m.1], Data::get_new(&pieces[m.0][m.1]));
    }

    pub fn calculate_moves(piece: &Piece, j: usize, i: usize) -> Vec<(usize, usize)> {
        let j = j as isize;
        let i = i as isize;
        match piece {
            Piece::Pawn(_) => {
                if j == 6 {
                    try_(vec![(j - 1, i), (j - 2, i)])
                } else {
                    try_(vec![(j - 1, i)])
                }
            }
            Piece::Knight(_) => try_(vec![
                (j - 2, i + 1),
                (j - 2, i - 1),
                (j + 2, i + 1),
                (j + 2, i - 1),
                (j - 1, i - 2),
                (j - 1, i + 2),
                (j + 1, i - 2),
                (j + 1, i + 2),
            ]),
            Piece::Bishop(_) => {
                let mut vec: Vec<(isize, isize)> = Vec::new();
                for diff in (-7..8).rev() {
                    vec.push((j + diff, i + diff));
                    vec.push((j + diff, i - diff));
                }
                try_(vec)
            }
            Piece::Rook(_) => {
                let mut vec: Vec<(isize, isize)> = Vec::new();
                for diff in (-7..8).rev() {
                    vec.push((j, i + diff));
                    vec.push((j + diff, i));
                }
                try_(vec)
            }
            Piece::Queen(_) => {
                let mut vec: Vec<(isize, isize)> = Vec::new();
                for diff in (-7..8).rev() {
                    vec.push((j, i + diff));
                    vec.push((j + diff, i));
                    vec.push((j + diff, i + diff));
                    vec.push((j + diff, i - diff));
                }
                try_(vec)
            }
            Piece::King(_) => try_(vec![
                (j, i + 1),
                (j, i - 1),
                (j + 1, i),
                (j - 1, i),
                (j + 1, i + 1),
                (j + 1, i - 1),
                (j - 1, i + 1),
                (j - 1, i - 1),
            ]),
            _ => Vec::new(),
        }
    }
}

fn try_(vec: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        if v.0 >= 0 && v.1 >= 0 {
            vec_safe.push((v.0 as usize, v.1 as usize));
        }
    }
    vec_safe
}