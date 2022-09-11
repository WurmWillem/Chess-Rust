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
                /*let mut vec_hor: Vec<(isize, isize)> = Vec::new();
                let mut vec_hor1: Vec<(isize, isize)> = Vec::new();
                let mut vec_ver: Vec<(isize, isize)> = Vec::new();
                let mut vec_ver1: Vec<(isize, isize)> = Vec::new();

                /*for diff in (-7..8).rev() {
                    vec_hor.push((j, i + diff));
                    vec_ver.push((j + diff, i));
                }*/
                /*for diff in (i + 1)..8 {
                    if diff != 0 {
                        vec_hor.push((j, i + diff));
                    }
                }
                let mut vec_hor = return_non_blocked_moves(pieces, return_safe_moves(vec_hor)); 
    
                for diff in (-7..i).rev() {
                    if diff != 0 {
                        vec_hor1.push((j, i + diff));
                    }
                }
                let mut vec_hor1 = return_non_blocked_moves(pieces, return_safe_moves(vec_hor1)); 
                for diff in (j + 1)..8 {
                    if diff != 0 {
                        vec_ver.push((j + diff, i));
                    }
                }
                let mut vec_ver = return_non_blocked_moves(pieces, return_safe_moves(vec_ver)); 
                for diff in (-7..j).rev() {
                    //println!("{diff}");
                    if diff != 0 {
                        vec_ver1.push((j + diff, i));
                    }
                    
                }
                let mut vec_ver1 = return_non_blocked_moves(pieces, return_safe_moves(vec_ver1)); 
                
                //println!("");
                vec_hor.append(&mut vec_hor1);
                vec_ver.append(&mut vec_ver1);
                vec_hor.append(&mut vec_ver);
                
                
                vec_hor*/*/
                let mut vec: Vec<(isize, isize)> = Vec::new();
                for diff in (-7..8).rev() {
                    vec.push((j, i + diff));
                    vec.push((j + diff, i));
                }
                return_safe_moves(vec)

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

fn return_non_blocked_moves(pieces: &Vec<Vec<Piece>>, vec: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        //println!("{} {}", v.0, v.1);
        if pieces[v.0][v.1] != Piece::None {
            println!("piece found at {:?}", v);
            break;
        }
        //println!("empty");
        vec_safe.push(*v);
    }
    for v in &vec_safe {
        //println!("{:?}", *v);
    }
    vec_safe
}

fn return_safe_moves(vec: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        if v.0 >= 0 && v.0 < 8 && v.1 >= 0 && v.1 < 8 {
            vec_safe.push((v.0 as usize, v.1 as usize));
            //println!("{} {}", v.0, v.1);
        }
    }
    //println!("");
    vec_safe
}
