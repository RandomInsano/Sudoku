extern crate rand;

//use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let mut pf = Playfeild::new();

    pf.set(0, 1, 3);
    pf.set(3, 1, 6);
    pf.set(0, 3, 5);
    pf.set(1, 0, 4);

    pf.print();
    
    println!("Possible:");
    for i in pf.find_possible(1, 1).iter() {
        print!("{},", i);
    }
    println!();

    println!("Hello, world!");
}

const P_WIDTH: usize = 10;
const P_HEIGHT: usize = 9;

struct Playfeild {
    board: [[u8; P_WIDTH]; P_HEIGHT],
}

impl Playfeild {
    fn new() -> Playfeild {
        Playfeild {
            board: [[0u8; P_WIDTH]; P_HEIGHT],
        }
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.board[y][x] = value;
    }

    // Find the possible options for this tile
    fn find_possible(self, x: usize, y: usize) -> Vec<u8> {
        let mut possible: HashSet<u8> = HashSet::from_iter(1..10);

        // Find all numbers in our current row
        for ix in 0 .. P_WIDTH {
            if self.board[y][ix] != 0 {
                possible.remove(&self.board[y][ix]);
            }
        }

        // Find all numbers in our current column
        for iy in 0 .. P_HEIGHT {
            if self.board[iy][x] != 0 {
                possible.remove(&self.board[iy][x]);
            }
        }

        Vec::<u8>::from_iter(possible.iter().cloned())
    }

    /*
    fn fill_random(&mut self) {
        let mut rng = thread_rng();

        for y in 0 .. P_HEIGHT {
            for x in 0 .. P_WIDTH {
                self.board[y][x] = x;
            }
        }
    }
    */

    fn print(&self) {
        println!("\nPlayfield:");
        for col in self.board.into_iter() {
            for row in col.into_iter() {
                print!(" {}", row);
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_possible() {
        let mut pf = Playfeild::new();

        pf.set(0, 1, 3);
        pf.set(3, 1, 6);
        pf.set(0, 3, 5);
        pf.set(1, 0, 4);

        let items = pf.find_possible(1, 1);
        for i in [1, 2, 5, 7, 8, 9].iter() {
            print!("{}-", i);
            assert!(items.contains(i));
        }
    }
}
