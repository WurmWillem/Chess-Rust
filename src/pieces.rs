use crate::{piece_data::Data, state::Side};

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

    pub fn calculate_moves(
        pieces: &Vec<Vec<Piece>>,
        piece: &Piece,
        j: usize,
        i: usize,
    ) -> Vec<(usize, usize)> {
        let j = j as isize;
        let i = i as isize;
        match piece {
            Piece::Pawn(_) => generate_pawn_moves(pieces, i, j),
            Piece::Knight(_) => return_safe_moves_vec(vec![
                (j - 2, i + 1),
                (j - 2, i - 1),
                (j + 2, i + 1),
                (j + 2, i - 1),
                (j - 1, i - 2),
                (j - 1, i + 2),
                (j + 1, i - 2),
                (j + 1, i + 2),
            ]),
            Piece::Bishop(_) => generate_bishop_moves(pieces, i, j),
            Piece::Rook(_) => generate_rook_moves(pieces, i, j),
            Piece::Queen(_) => {
                let mut bishop_moves = generate_bishop_moves(pieces, i, j);
                let mut rook_moves = generate_rook_moves(pieces, i, j);

                bishop_moves.append(&mut rook_moves);
                bishop_moves
            }
            Piece::King(_) => return_safe_moves_vec(vec![
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

fn generate_pawn_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = Vec::new();

    if Data::get_side(&pieces[j as usize][i as usize]) == Side::White {
        if pieces[return_if_positive(j, -1)][i as usize] == Piece::None {
            moves.append(&mut return_safe_moves_vec(vec![(j - 1, i)]));
        }
        if j == 6 && pieces[return_if_positive(j, -2)][i as usize] == Piece::None{
            moves.append(&mut return_safe_moves_vec(vec![(j - 2, i)]));
        }
        if Data::get_side(&pieces[return_if_positive(j, -1)][i as usize + 1]) == Side::Black {
            moves.append(&mut return_safe_moves_vec(vec![(j - 1, i + 1)]));
        }
        println!("{} {}", j, i);
        if Data::get_side(&pieces[return_if_positive(j, -1)][return_if_positive(i, -1)]) == Side::Black {
            moves.append(&mut return_safe_moves_vec(vec![(j - 1, i - 1)]));
        }
    } else if Data::get_side(&pieces[j as usize][i as usize]) == Side::Black {
        if pieces[j as usize + 1][i as usize] == Piece::None {
            moves.append(&mut return_safe_moves_vec(vec![(j + 1, i)]));
        }
        if j == 1 && pieces[j as usize + 2][i as usize] == Piece::None {
            moves.append(&mut return_safe_moves_vec(vec![(j + 2, i)]));
        }
        if Data::get_side(&pieces[j as usize + 1][i as usize + 1]) == Side::White {
            moves.append(&mut return_safe_moves_vec(vec![(j + 1, i + 1)]));
        }
        else if Data::get_side(&pieces[j as usize + 1][i as usize - 1]) == Side::White {
            moves.append(&mut return_safe_moves_vec(vec![(j + 1, i - 1)]));
        }
    } else {
        panic!("side is unknown");
    }
    
    moves
}

fn generate_bishop_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let mut right_up: Vec<(isize, isize)> = Vec::new();
    let mut left_up: Vec<(isize, isize)> = Vec::new();
    let mut left_down: Vec<(isize, isize)> = Vec::new();
    let mut right_down: Vec<(isize, isize)> = Vec::new();

    let mut x = 1;
    for _ in (i + 1)..8 {
        right_up.push((j + x, i + x));
        right_down.push((j - x, i + x));
        x += 1;
    }

    let mut x = 1;
    for _ in 1..(i + 1) {
        left_up.push((j + x, i - x));
        left_down.push((j - x, i - x));
        x += 1;
    }

    let mut right_up = return_non_blocked_moves(pieces, return_safe_moves_vec(right_up));
    let mut right_down = return_non_blocked_moves(pieces, return_safe_moves_vec(right_down));
    let mut left_up = return_non_blocked_moves(pieces, return_safe_moves_vec(left_up));
    let mut left_down = return_non_blocked_moves(pieces, return_safe_moves_vec(left_down));

    right_up.append(&mut left_up);
    right_down.append(&mut left_down);
    right_up.append(&mut right_down);

    let vec_all = right_up.clone();
    vec_all
}

fn generate_rook_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let mut vec_right: Vec<(isize, isize)> = Vec::new();
    let mut vec_left: Vec<(isize, isize)> = Vec::new();
    let mut vec_up: Vec<(isize, isize)> = Vec::new();
    let mut vec_down: Vec<(isize, isize)> = Vec::new();

    let mut x = 1;
    for _ in (i + 1)..8 {
        vec_right.push((j, i + x));
        x += 1;
    }

    let mut x = 1;
    for _ in 1..(i + 1) {
        vec_left.push((j, i - x));
        x += 1;
    }

    let mut x = 1;
    for _ in (j + 1)..8 {
        vec_up.push((j + x, i));
        x += 1;
    }

    let mut x = 1;
    for _ in 1..(j + 1) {
        vec_down.push((j - x, i));
        x += 1;
    }

    let mut vec_right = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_right));
    let mut vec_left = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_left));
    let mut vec_up = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_up));
    let mut vec_down = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_down));

    vec_right.append(&mut vec_left);
    vec_up.append(&mut vec_down);
    vec_right.append(&mut vec_up);

    let vec_all = vec_right.clone();
    vec_all
}

fn return_non_blocked_moves(
    pieces: &Vec<Vec<Piece>>,
    vec: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        //println!("{:?}", v);
        if pieces[v.0][v.1] != Piece::None {
            //println!("piece found at {:?}", v);
            break;
        }

        vec_safe.push(*v);
    }
    vec_safe
}

fn return_if_positive(x: isize, diff: isize) -> usize{
    if x + diff >= 0 {
        return (x + diff) as usize
    }
    x as usize
}

fn return_safe_moves_vec(vec: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        if v.0 >= 0 && v.0 < 8 && v.1 >= 0 && v.1 < 8 {
            vec_safe.push((v.0 as usize, v.1 as usize));
        }
    }
    vec_safe
}
