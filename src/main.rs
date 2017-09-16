extern crate rand;

use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let mut pf = Playfeild::new();
    let mut guesses: usize = 1; // Hoomans counts at one

    pf.set(1, 0, 6);
    pf.set(2, 1, 7);
    while (!pf.filled()) {
        guesses += 1;
        pf.reset();
        pf.fill_possible();
    }

    pf.print();
    println!("Found a result in {} guesses.", guesses);
}

const P_WIDTH: usize = 9;
const P_HEIGHT: usize = 9;

struct Playfeild {
    board: [[u8; P_WIDTH]; P_HEIGHT],
    random: Box<Rng>,
}

impl Playfeild {
    fn new() -> Playfeild {
        Playfeild {
            board: [[0u8; P_WIDTH]; P_HEIGHT],
            random: Box::new(thread_rng()),
        }
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.board[y][x] = value;
    }

    // Find the possible options for this tile
    fn find_possible(&self, x: usize, y: usize) -> Vec<u8> {
        let mut possible: HashSet<u8> = HashSet::from_iter(1..10);

        assert!(x < P_WIDTH);
        assert!(y < P_HEIGHT);

        // Find all numbers in our current row
        for ix in 0 .. P_WIDTH {
            let value = self.board[y][ix];
            if value != 0 {
                possible.remove(&value);
            }
        }

        // Find all numbers in our current column
        for iy in 0 .. P_HEIGHT {
            let value = self.board[iy][x];
            if value != 0 {
                possible.remove(&value);
            }
        }

        // Find all numbers in our quadrand
        let qx = x / 3; // Mush X and Y down
        let qy = y / 3; // So we're clamped to a section
        for iy in qy * 3 .. (qy + 1) * 3 {
            for ix in qx * 3 .. (qx + 1) * 3 {
                let value = self.board[iy][ix];
                if value != 0 {
                    possible.remove(&value);
                }
            }
        }

        Vec::<u8>::from_iter(possible.iter().cloned())
    }

    fn fill_possible(&mut self) {
        for y in 0 .. P_HEIGHT {
            for x in 0 .. P_WIDTH {
                if self.board[y][x] == 0 {
                    let mut possible = self.find_possible(x, y);
                    possible.sort();

                    // TODO: Purposely randomizing for now to brute-force solution
                    self.random.shuffle(&mut possible);

                    if let Some(value) = possible.iter().next() {
                        self.board[y][x] = *value;
                    } else {
                        
                    }
                }
            }
        }
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

    // TODO: Implement an iterator over the collection to
    // save us from all these for loops
    fn reset(&mut self) {
        for y in 0 .. P_HEIGHT {
            for x in 0 .. P_WIDTH {
                self.board[y][x] = 0
            }
        }
    }

    fn filled(&self) -> bool {
        for y in 0 .. P_HEIGHT {
            for x in 0 .. P_WIDTH {
                if self.board[y][x] == 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn print(&self) {
        println!("\nPlayfield:");
        for col in self.board.into_iter() {
            for row in col.into_iter() {
                if *row > 0 {
                    print!(" {}", row);
                } else {
                    print!(" _");
                }
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
