use itertools::Itertools;

pub(crate) struct Cage {
    layers: Vec<Vec<Option<u8>>>,
}

impl Cage {
    pub(crate) fn new() -> Self {
        Self {
            layers: vec![vec![None; 8]; 3],
        }
    }

    pub(crate) fn drop(&mut self, cube: u8, column: usize) -> Result<(), &'static str> {
        if column > 7 {
            return Err("Column is out of bounds; column should be between 0 and 7");
        }
        assert!(cube < 6);

        let c = &mut self.layers[0][column];
        if c.is_some() {
            return Err("Column is full");
        }
        *c = Some(cube);

        self.do_gravity();
        Ok(())
    }

    pub(crate) fn rotate(&mut self, layer: usize, clockwise: bool) {
        assert!(layer <= 2);

        let layer = &mut self.layers[layer];

        if clockwise {
            layer.rotate_right(1);
        } else {
            layer.rotate_left(1);
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
                (0, 1, 2), (3, 4, 5), (6, 7, 8), // Horizontal
                (0, 3, 6), (1, 4, 7), (2, 5, 8), // Vertical
                (0, 4, 8), (2, 4, 6), // Diagonal
            ] {
                if let Some(a) = self.get_from_location(side, a) {
                    if Some(a) == self.get_from_location(side, b)
                        && Some(a) == self.get_from_location(side, c) {
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

    pub(crate) fn string_representation_2d(&self) -> String {
        self.layers
            .iter()
            .map(|layer| {
                layer.iter().map(|cube| {
                    match cube {
                        Some(c) => c.to_string(),
                        None => " ".to_string(),
                    }
                }).collect::<String>()
            })
            .join("\n")
    }
}
