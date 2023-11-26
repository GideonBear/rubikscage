use crate::cage::Cage;
use crate::player::Player;
use clap::Parser;
use clap_num::number_range;

fn players_parser(s: &str) -> Result<u8, String> {
    number_range(s, 1, 4)
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_parser=players_parser)]
    players: u8,
}

pub fn run<P: Player>() {
    let args = Args::parse();
    let mut cage = Cage::new();

    let stocks: Vec<Vec<u8>> = match args.players {
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

    let mut players: Vec<P> = stocks.into_iter().map(P::new).collect();

    'main: loop {
        for player in &mut players {
            player.make_move(&mut cage);
            if let Some(won_color) = cage.check_win() {
                let won_player = match args.players {
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
