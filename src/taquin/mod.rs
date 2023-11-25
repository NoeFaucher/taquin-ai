pub mod ai;

use std::{hash::Hash, cmp::min, num};

use rand::{seq::SliceRandom, distributions::uniform::UniformSampler};



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Num(u32),
}

impl Cell {
    fn get_value(self) -> u32 {
        match self {
            Self::Num(x) => x,
            Self::Empty => 0,
        } 
    }

    fn get_string_value(&self) -> String {
        match self {
            Self::Num(x) => x.to_string(),
            Self::Empty => String::from(" "),
        }
    }

}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Taquin {
    board: Vec<Vec<Cell>>,
    empty_coord: (usize,usize),
    size: usize,
}


impl Taquin {
    
    pub fn new(size: usize) -> Taquin {
        assert!(size>=2);

        let mut mat = vec![vec![Cell::Empty;size];size];
        let mut empty_coord = (0,0); 

        for val in 1..=size*size {
            let index = val - 1;

            let x = (index as u32 % size as u32) as usize;
            let y = (index as u32 / size as u32) as usize;

            if val == size*size {
                mat[x][y] = Cell::Empty;
                empty_coord = (x,y);
            }else {
                mat[x][y] = Cell::Num(val as u32);
            }
        }

        Taquin {
            board:  mat,
            empty_coord: empty_coord,
            size: size,
            }
    }

    pub fn new_rand(size: usize) -> Taquin {
        assert!(size>=2);
        
        let mut t = Self::new(size);

        let n = size*size*1000;

        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let mouvs = t.available_move();
            t.make_move(*mouvs.choose(&mut rng).unwrap());
        }
        
        return t;
    }

    pub fn show(&self) {

        for j in 0..self.size {
            print!("|");
            for i in 0..self.size {
                let val = self.board[i][j].get_string_value();
                print!("{:>5}|",val);
            }
            println!("");
        }
        println!("");
    }

    /** Calculate the heuristic of Hamming (distance) between 2 taquin board
     * 
     */
    pub fn heuristic_hamming(t1: &Taquin, t2: &Taquin) -> u32 {
        assert!(t1.size == t2.size,"Board has to be of the same size.");

        let mut count = 0;

        for i in 0..t1.size {
            for j in 0..t1.size {
                if t1.board[i][j] != t2.board[i][j] {
                    count += 1;
                }
            }
        }

        return count;
    }

    /** Calculate the heuristic of Manhattan (distance) between 2 taquin board
     * 
     */
    pub fn heuristic_manhattan(t1: &Taquin, t2: &Taquin) -> u32 {
        assert_eq!(t1.size,t2.size,"taquin must be of the same size");
        let mut res = 0;
        
        for i in 0..t1.size {
            for j in 0..t1.size {
                let cell = t1.board[i][j];

                for x in 0..t2.size  {
                    for y in 0..t2.size {
                        if cell == t2.board[x][y] {
                            res += ((i as i32 - x as i32).abs() + (j as i32 - y as i32).abs()) as u32;
                        }
                    }
                }
            }
        }

        return res;
    }

    /** Do a move on the board
     * return true if the move was successful
     */
    pub fn make_move(&mut self, m: Move) -> bool {

        if ! self.available_move().contains(&m) {
            println!("Illegal move : {:?}!",m);
            return false;
        }


        match m {
            Move::Up => self.swap((self.empty_coord.0 ,self.empty_coord.1 -1)),
            Move::Left => self.swap((self.empty_coord.0 -1 ,self.empty_coord.1)),
            Move::Down => self.swap((self.empty_coord.0 ,self.empty_coord.1 +1)),
            Move::Right => self.swap((self.empty_coord.0 +1 ,self.empty_coord.1)),
        }

        return true;
    }

    /** Do the oposite of the given move
     * 
     */
    pub fn reverse_move(&mut self, m: Move) -> bool {

        return match m {
            Move::Up => self.make_move(Move::Down),
            Move::Left => self.make_move(Move::Right),
            Move::Down => self.make_move(Move::Up),
            Move::Right => self.make_move(Move::Left),
        };


    }

    /** 
     * Returns all the available moves
     */
    pub fn available_move(&self) -> Vec<Move> {
        let mut moves = Move::get_all_move();

        if self.empty_coord.0 == 0 {
            let index = moves.iter().position(|x| *x == Move::Left).unwrap();
            moves.remove(index);
        }
        
        if self.empty_coord.0 == self.size-1 {
            let index = moves.iter().position(|x| *x == Move::Right).unwrap();
            moves.remove(index);
        }
        
        if self.empty_coord.1 == 0 {
            let index = moves.iter().position(|x| *x == Move::Up).unwrap();
            moves.remove(index);
        }
        
        if self.empty_coord.1 == self.size-1 {
            let index = moves.iter().position(|x| *x == Move::Down).unwrap();
            moves.remove(index);
        }

        return moves;
    }

    /** Swap the empty cell with the cell of the given coordinate 
     * 
     */
    fn swap(&mut self, coord: (usize,usize)) {
        self.board[self.empty_coord.0][self.empty_coord.1] = self.board[coord.0][coord.1];
        self.board[coord.0][coord.1] = Cell::Empty;
        self.empty_coord = coord;
    }

}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    Up,
    Left,
    Down,
    Right
}

impl Move {

    fn get_all_move() -> Vec<Self> {
        return vec!(Move::Up,Move::Left,Move::Down,Move::Right);
    }
    
}

