use crate::ant::Ant;

pub struct Chunk {
    pub pos_x: u32,
    pub pos_y: u32,
    pub ants: Vec<Ant>,
}

impl Chunk {
    pub fn is_chunk_empty(&self) -> bool {
        return self.ants.is_empty();
    }

}
