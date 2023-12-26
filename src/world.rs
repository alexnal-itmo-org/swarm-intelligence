use crate::chunk::Chunk;

pub struct World {
    pub chunks: Vec<Box<Chunk>>,
    size_x: i32,
    size_y: i32,
}

impl World {
    pub fn run(&self) {
        for chunk_iter in &self.chunks {
            if !Chunk::is_chunk_filled(chunk_iter) {
                //TODO: chunk scheduler
            }
        }
    }
}

pub fn init_world(x_chunks: i32, y_chunks: i32) -> World {
    let mut world = World { chunks: Vec::new(), size_x: x_chunks, size_y: y_chunks };

    for x in 1..x_chunks {
        for y in 1..y_chunks {
            let mut chunk = Chunk { pos_x: x, pos_y: y, ants: Vec::new() };
            world.chunks.push(Box::new(chunk));
        }
    }

    return world;
}