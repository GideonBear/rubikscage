use std::iter::zip;
use crate::Cage;
use crate::cage::Move;
use crate::Player;

pub fn run<P: Player>(num_players: u8) {
    // TODO: rename cube -> color
    let mut cage = Cage::new();

    let mut stocks: Vec<Vec<u8>> = match num_players {
        2 => vec![
            vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5],
        ],
        3 => vec![
            vec![0, 0, 0, 0, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 3, 3, 3, 3],
            vec![4, 4, 4, 4, 5, 5, 5, 5],
        ],
        4 => vec![
            vec![0, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![3, 3, 3, 3],
        ],
        _ => unreachable!(),
    };

    let mut players: Vec<P> = stocks.iter().map(P::new).collect();

    'main: loop {
        for (player, stock) in zip(&mut players, &mut stocks) {
            let move_ = player.get_move(&cage);
            if let Move::Drop(cube, _) = move_ {
                let index = match stock.iter().position(|&x| x == cube) {
                    Some(i) => i,
                    None => panic!("Player violation: Cube not in stock")
                };
                stock.swap_remove(index);
            }
            cage.make_move(move_);
            if let Some(won_color) = cage.check_win() {
                let won_player = match num_players {
                    2 => match won_color {
                        0..=2 => 0,
                        3..=5 => 1,
                        _ => unreachable!(),
                    },
                    3 => match won_color {
                        0..=1 => 0,
                        2..=3 => 1,
                        4..=5 => 2,
                        _ => unreachable!(),
                    },
                    4 => match won_color {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
                players[won_player].won(won_color);
                break 'main;
            }
        }
    }
}
