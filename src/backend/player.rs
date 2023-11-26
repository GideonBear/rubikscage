use crate::Cage;

pub trait Player {
    fn new(stock: Vec<u8>) -> Self;
    fn make_move(&mut self, cage: &mut Cage);
    fn won(&self, won_color: u8);
}
