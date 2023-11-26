use crate::input_player::InputPlayer;
use rubikscage::run::run;

mod input_player;

fn main() {
    run::<InputPlayer>()
}
