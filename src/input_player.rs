use crate::cage::Cage;
use crate::player::Player;
use itertools::Itertools;
use std::io::{stdin, Write};

fn input() -> String {
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    input_string.trim().to_string()
}

#[derive(Debug)]
pub(crate) struct InputPlayer {
    name: String,
    stock: Vec<u8>,
}

impl Player for InputPlayer {
    fn new(stock: Vec<u8>) -> Self {
        print!("Enter your name: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        let name = input();
        Self {
            name,
            stock
        }
    }

    fn make_move(&mut self, cage: &mut Cage) {
        loop {
            println!("{}", cage.string_representation_2d());
            println!("{}, what's your move?", self.name);
            let input_string = input();
            match input_string.chars().collect_vec()[..] {
                ['d', ccube, ccolumn] => {
                    let cube: u8;
                    match ccube.to_digit(10) {
                        None => {
                            println!("Invalid cube");
                            continue
                        }
                        Some(c) => cube = c as u8,
                    }
                    let column: usize;
                    match ccolumn.to_digit(10) {
                        None => {
                            println!("Invalid column");
                            continue
                        }
                        Some(c) => column = c as usize,
                    }
                    let index = match self.stock.iter().position(|&x| x == cube) {
                        Some(i) => i,
                        None => {
                            println!("Cube not in stock");
                            continue
                        }
                    };
                    self.stock.swap_remove(index);
                    if let Err(s) = cage.drop(cube, column) {println!("{}", s) }
                }
                ['r', layer, cw] => {
                    let layer = match layer {
                        'b' => 0,
                        'm' => 1,
                        't' => 2,
                        _ => {
                            println!("Invalid layer");
                            continue
                        }
                    };
                    let cw = match cw {
                        'w' => true,
                        'c' => false,
                        _ => {
                            println!("Invalid cw");
                            continue
                        }
                    };
                    cage.rotate(layer, cw);
                }
                ['f'] => {
                    cage.flip();
                }
                _ => {
                    println!("Invalid input");
                    continue
                }
            }
            break
        }
    }

    fn won(&self, won_color: u8) {
        println!("{} won with color {}!", self.name, won_color);
    }
}
