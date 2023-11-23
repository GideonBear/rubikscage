use crate::cube::Cube;

type Column = (Option<Cube>, Option<Cube>, Option<Cube>);

struct Cage {
    spaces: Vec<Column>
}

impl Cage {
    fn new() -> Self {
        Self {
            spaces: vec![(None, None, None); 9]
        }
    }

    fn drop(&mut self, cube: Cube, column: usize) -> Result<(), &'static str> {
        if column > 8 {
            return Err("Column is out of bounds; column should be between 0 and 8")
        }
        let c = &mut self.spaces[column].2;
        if c.is_some() {
            return Err("Column is full")
        }
        *c = Some(cube);
        self.do_gravity();
        Ok(())
    }

    fn rotate_bottom(cw: bool) {

    }
}
