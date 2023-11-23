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

        let c = &mut self.layers[2][column];
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
            for (b_layer, t_layer) in [(0, 1), (1, 2)] {
                let t_cube = self.layers[t_layer][column].take();
                self.layers[b_layer][column] = t_cube;
            }
        }
    }

    pub(crate) fn check_win(&self) -> Option<u8> {

    }
}
