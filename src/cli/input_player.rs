use itertools::Itertools;
use rubikscage::Cage;
use rubikscage::Player;
use std::io::{stdin, Write};
use rubikscage::cage::Move;

fn input() -> String {
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    if input_string.is_empty() {
        panic!("Zero-length input (EOF) while reading stdin");
    }
    input_string.trim().to_string()
}

#[derive(Debug)]
pub(crate) struct InputPlayer {
    name: String,
    stock: Vec<u8>,
}

impl Player for InputPlayer {
    fn new(stock: &Vec<u8>) -> Self {
        print!("Enter your name: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        let name = input();
        let stock = stock.to_owned();
        Self { name, stock }
    }

    fn get_move(&mut self, cage: &Cage) -> Move {
        loop {
            println!("{}", cage.string_representation_2d());
            println!("{}, what's your move?", self.name);
            let input_string = input();
            match input_string.chars().collect_vec()[..] {
                ['d', ccube, ccolumn] => {
                    let cube: u8 = match ccube.to_digit(10) {
                        None => {
                            println!("Cube must be a number");
                            continue;
                        }
                        Some(c) => c as u8,
                    };
                    let column: usize = match ccolumn.to_digit(10) {
                        None => {
                            println!("Column must be a number");
                            continue;
                        }
                        Some(c) => {
                            if c >= 8 {
                                println!("Column must be between 0 and 7");
                                continue
                            }
                            c as usize
                        }
                    };
                    let index = match self.stock.iter().position(|&x| x == cube) {
                        Some(i) => i,
                        None => {
                            println!("Cube not in stock");
                            continue;
                        },
                    };
                    self.stock.swap_remove(index);
                    // TODO: check if column is not blocked
                    return Move::Drop(cube, column)
                }
                ['r', layer, cw] => {
                    let layer = match layer {
                        't' => 0,
                        'm' => 1,
                        'b' => 2,
                        _ => {
                            println!("Invalid layer");
                            continue;
                        }
                    };
                    let cw = match cw {
                        'r' => true,
                        'l' => false,
                        _ => {
                            println!("Invalid cw");
                            continue;
                        }
                    };
                    return Move::Rotate(layer, cw);
                }
                ['f'] => {
                    return Move::Flip;
                }
                _ => {
                    println!("Invalid input");
                    continue;
                }
            }
        }
    }

    fn won(&self, won_color: u8) {
        println!("{} won with color {}!", self.name, won_color);
    }
}
