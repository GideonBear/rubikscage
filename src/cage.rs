use crate::cube::Cube;

struct Cage {
    layers: Vec<Vec<Option<Cube>>>
}

impl Cage {
    fn new() -> Self {
        Self {
            layers: vec![vec![None; 8]; 3]
        }
    }

    fn drop(&mut self, cube: Cube, column: usize) -> Result<(), &'static str> {
        if column > 7 {
            return Err("Column is out of bounds; column should be between 0 and 7")
        }

        let c = &mut self.layers[2][column];
        if c.is_some() {
            return Err("Column is full")
        }
        *c = Some(cube);

        self.do_gravity();
        Ok(())
    }

    fn rotate(&mut self, layer: usize, clockwise: bool) -> Result<(), &'static str> {
        if layer > 2 {
            return Err("Layer is out of bounds; layer should be between 0 and 2")
        }

        let mut layer = &mut self.layers[layer];

        if clockwise {
            layer.rotate_right(1);
        } else {
            layer.rotate_left(1);
        }

        self.do_gravity();
        Ok(())
    }

    fn do_gravity(&mut self) {
        for column in 0..8 {
            for (b_layer, t_layer) in [(0, 1), (1, 2)] {
                let t_cube = self.layers[t_layer][column].take();
                self.layers[b_layer][column] = t_cube;
            }
        }
    }
}
