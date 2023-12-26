use crate::ant::Ant;

pub struct Chunk {
    pub pos_x: i32,
    pub pos_y: i32,
    pub ants: Vec<Ant>,
}

impl Chunk {
    pub fn run(&self) {
        for ant in self.ants.iter() {
            ant.run();
        }
    }

    pub fn is_chunk_filled(&self) -> bool {
        return self.ants.is_empty();
    }
}
