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
            Piece::Pawn(_) => {
                if Data::get_side(&pieces[j as usize][i as usize]) == Side::White {
                    if j == 6 {
                        return_safe_moves(vec![(j - 1, i), (j - 2, i)])
                    } else {
                        return_safe_moves(vec![(j - 1, i)])
                    }
                } else if Data::get_side(&pieces[j as usize][i as usize]) == Side::Black {
                    if j == 1 {
                        return_safe_moves(vec![(j + 1, i), (j + 2, i)])
                    } else {
                        return_safe_moves(vec![(j + 1, i)])
                    }
                } else {
                    panic!("side is unknown");
                }
            }
            Piece::Knight(_) => return_safe_moves(vec![
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
                return_safe_moves(vec)
            }
            Piece::Rook(_) => {
                let mut vec_right: Vec<(isize, isize)> = Vec::new();
                let mut vec_left: Vec<(isize, isize)> = Vec::new();
                let mut vec_up: Vec<(isize, isize)> = Vec::new();
                let mut vec_down: Vec<(isize, isize)> = Vec::new();

                let mut x = 1;
                for _ in (i + 1)..8 {
                    vec_right.push((j, i + x));
                    x += 1;
                }
                let mut vec_right = return_non_blocked_moves(pieces, return_safe_moves(vec_right));

                let mut x = 1;
                for _ in 1..(i + 1) {
                    vec_left.push((j, i - x));
                    x += 1;
                }
                let mut vec_left = return_non_blocked_moves(pieces, return_safe_moves(vec_left));

                let mut x = 1;
                for _ in (j + 1)..8 {
                    vec_up.push((j + x, i));
                    x += 1;
                }
                let mut vec_up = return_non_blocked_moves(pieces, return_safe_moves(vec_up));

                let mut x = 1;
                for _ in 1..(j + 1) {
                    vec_down.push((j - x, i));
                    x += 1;
                }
                let mut vec_down = return_non_blocked_moves(pieces, return_safe_moves(vec_down));

                /*for v in &vec_right {
                    println!("right safe and not blocked is {:?}", v);
                }
                println!("");
                for v in &vec_left {
                    println!("left safe and not blocked is {:?}", v);
                }
                println!("");
                for v in &vec_up {
                    println!("up safe and not blocked is {:?}", v);
                }
                println!("");
                for v in &vec_down {
                    println!("down safe and not blocked is {:?}", v);
                }*/
                println!("");
                
                vec_right.append(&mut vec_left);
                vec_up.append(&mut vec_down);
                vec_right.append(&mut vec_up);


                vec_right
                /*let mut vec: Vec<(isize, isize)> = Vec::new();
                for diff in (-7..8).rev() {
                    vec.push((j, i + diff));
                    vec.push((j + diff, i));
                }
                return_safe_moves(vec)*/
            }
            Piece::Queen(_) => {
                let mut vec: Vec<(isize, isize)> = Vec::new();
                for diff in (-7..8).rev() {
                    vec.push((j, i + diff));
                    vec.push((j + diff, i));
                    vec.push((j + diff, i + diff));
                    vec.push((j + diff, i - diff));
                }
                return_safe_moves(vec)
            }
            Piece::King(_) => return_safe_moves(vec![
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
    for v in &vec_safe {
        //println!("non blocked is safe is {:?}", *v);
    }
    println!("");
    vec_safe
}

fn return_safe_moves(vec: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        if v.0 >= 0 && v.0 < 8 && v.1 >= 0 && v.1 < 8 {
            vec_safe.push((v.0 as usize, v.1 as usize));
        }
    }
    
    vec_safe
}
