use sfml::system::Vector2u;
use crate::chunk::Chunk;
use crate::ant::Ant;

use std::f32::consts::PI;

pub const SCALER: f32 = 100.0;

pub struct World {
    pub chunks: Vec<Box<Chunk>>,
    size_x: u32,
    size_y: u32,
}

impl World {
    pub fn run(&self) {
        for chunk_iter in &self.chunks {
            if chunk_iter.is_chunk_empty() {
                continue;
            }
            //TODO:run impl
        }
    }

    pub fn get_chunk(&mut self, x: i32, y: i32) -> Option<&mut Box<Chunk>> {
        match self.chunks.get_mut((x * self.size_y + y) as usize) {
            Some(chunk) => Some(chunk),
            None => None,
        }
    }
    pub fn get_next_fluctuation() -> f32 {
        return 0.0;
    }
    pub fn get_default_ant_view() -> f32 {
        return 0.0;
    }
    pub fn get_ant_live_time() -> u32 {
        return 100;
    }
    pub fn add_ant(&mut self, x: i32, y: i32) -> bool {
        if let Some(chunk) = self.get_chunk(x, y) {
            let mut ant = Ant {
                rel_pos_x: x,
                rel_pos_y: y,
                dg_fluctuation: Self::get_next_fluctuation(),
                dg_view: Self::get_default_ant_view(),
                life_time: Self::get_ant_live_time(),
                food_taken: 0,
            };

            chunk.ants.push(ant);
            return true;
        }
        return false;
    }
    pub fn move_ant_to_chunk(&mut self, x: i32, y: i32){

    }

    pub fn ant_move(x: i32, y: i32, dg: f32, dgfluc: f32) -> (i32, i32) {
        let radians = (dg + dgfluc) as f32 * PI / 180.0;
        let dx = radians.cos() * SCALER;
        let dy = radians.sin() * SCALER;

        return (x as i32 + dx as i32, y as i32 + dy as i32);
    }
    pub fn ant_iter_move(&mut self, ant: &mut Ant) {
        let (dx, dy) = Self::ant_move(ant.rel_pos_x,
                                      ant.rel_pos_y,
                                      ant.dg_view,
                                      ant.dg_fluctuation);

        ant.rel_pos_x = dx;
        ant.rel_pos_y = dy;

        self.add_ant(dx, dy, &mut ant);
    }
}

pub fn init_world(x_chunks: u32, y_chunks: u32) -> World {
    let mut world = World { chunks: Vec::new(), size_x: x_chunks, size_y: y_chunks };

    for x in 0..x_chunks {
        for y in 0..y_chunks {
            let mut chunk = Chunk { pos_x: x, pos_y: y, ants: Vec::new() };
            world.chunks.push(Box::new(chunk));
        }
    }

    return world;
}
