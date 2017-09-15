extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let mut pf = Playfeild::new();
    pf.fill_random();
    pf.print();
    println!("Hello, world!");
}

const P_WIDTH: usize = 9;
const P_HEIGHT: usize = 9;

struct Playfeild {
    board: [[u8; P_WIDTH]; P_HEIGHT],
}

impl Playfeild {
    fn new() -> Playfeild {
        Playfeild {
            board: [[0u8; 9]; 9],
        }
    }

    fn fill_random(&mut self) {
        let mut rng = thread_rng();

        for y in 0 .. P_HEIGHT {
            for x in 0 .. P_WIDTH {
                self.board[x][y] = rng.gen_range::<u8>(0, 9)
            }
        }
    }

    fn print(self) {
        println!("\nPlayfield:");
        for row in self.board.into_iter() {
            for col in row.into_iter() {
                print!(" {}", col);
            }
            println!();
        }
        println!();
    }
}