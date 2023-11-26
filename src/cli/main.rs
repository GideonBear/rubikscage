use crate::input_player::InputPlayer;
use rubikscage::run;
use clap::Parser;
use clap_num::number_range;

mod input_player;

fn players_parser(s: &str) -> Result<u8, String> {
    number_range(s, 1, 4)
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_parser=players_parser)]
    players: u8,
}

fn main() {
    let args = Args::parse();

    run::<InputPlayer>(args.players);
}
