use crate::Cage;
use crate::cage::Move;

pub trait Player {
    fn new(stock: &Vec<u8>) -> Self;
    fn get_move(&mut self, cage: &Cage) -> Move;
    fn won(&self, won_color: u8);
}
