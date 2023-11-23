use crate::cage::Cage;

pub(crate) trait Player {
    fn new(stock: Vec<u8>) -> Self;
    fn make_move(&self, cage: &mut Cage);
    fn won(&self, won_color: u8);
}
