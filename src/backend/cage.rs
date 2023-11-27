use itertools::Itertools;

pub struct Cage {
    layers: Vec<Vec<Option<u8>>>,
}

impl Cage {
    pub(crate) fn new() -> Self {
        Self {
            layers: vec![vec![None; 8]; 3],
        }
    }

    pub(crate) fn make_move(&mut self, move_: Move) {
        match move_ {
            Move::Drop(cube, column) => self.drop(cube, column),
            Move::Rotate(layer, clockwise) => self.rotate(layer, clockwise),
            Move::Flip => self.flip(),
        }
    }

    pub(crate) fn drop(&mut self, cube: u8, column: usize) {
        assert!(column < 8, "Player violation: assert column < 8");
        assert!(cube < 6, "Player violation: assert cube < 6");

        let c = &mut self.layers[0][column];
        assert!(c.is_none(), "Player violation: Column is full");
        *c = Some(cube);

        self.do_gravity();
    }

    pub fn column_full(&self, column: usize) -> bool {
        self.layers[0][column].is_some()
    }

    pub(crate) fn rotate(&mut self, layer: usize, clockwise: bool) {
        assert!(layer < 3, "Player violation: assert layer < 3");

        let layer = &mut self.layers[layer];

        if clockwise {
            layer.rotate_right(2);
        } else {
            layer.rotate_left(2);
        }

        self.do_gravity();
    }

    pub(crate) fn flip(&mut self) {
        self.layers.reverse();
        self.do_gravity();
    }

    fn do_gravity(&mut self) {
        for column in 0..8 {
            for (t_layer, b_layer) in [(1, 2), (0, 1), (1, 2)] {
                let t_cube = self.layers[t_layer][column];
                let b_cube = self.layers[b_layer][column];
                if t_cube.is_some() && b_cube.is_none() {
                    self.layers[b_layer][column] = self.layers[t_layer][column].take();
                }
            }
        }
    }

    pub(crate) fn check_win(&self) -> Option<u8> {
        for side in 0..4 {
            for (a, b, c) in [
                // Horizontal
                (0, 1, 2),
                (3, 4, 5),
                (6, 7, 8),
                // Vertical
                (0, 3, 6),
                (1, 4, 7),
                (2, 5, 8),
                // Diagonal
                (0, 4, 8),
                (2, 4, 6),
            ] {
                if let Some(a) = self.get_from_location(side, a) {
                    if Some(a) == self.get_from_location(side, b)
                        && Some(a) == self.get_from_location(side, c)
                    {
                        return Some(a);
                    }
                }
            }
        }
        None
    }

    fn get_from_location(&self, side: u8, location: u8) -> Option<u8> {
        match location {
            0..=2 => self.layers[0][(location + side) as usize],
            3..=5 => self.layers[1][(location - 3 + side) as usize],
            6..=8 => self.layers[2][(location - 6 + side) as usize],
            _ => unreachable!(),
        }
    }

    pub fn string_representation_2d(&self) -> String {
        self.layers
            .iter()
            .map(|layer| {
                layer
                    .iter()
                    .map(|cube| match cube {
                        Some(c) => c.to_string(),
                        None => " ".to_string(),
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

pub enum Move {
    Drop(u8, usize),
    Rotate(usize, bool),
    Flip,
}
