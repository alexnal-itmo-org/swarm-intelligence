extern crate sfml;

use sfml::{
    graphics::{ RenderWindow, RenderTarget, Sprite, Shape, Texture, Image, Drawable, Transformable, Color, CircleShape },
    system::{ Clock, Time, Vector2i, Vector2f },
    window::{ VideoMode, Style, ContextSettings, Event, Key},
};

pub struct Ant {
    rel_pos_x: u8,
    rel_pos_y: u8,
    life_time: u16,
    food_taken: u16
}
impl Ant {
    pub fn run(&self){
        //TODO: life cycle
    }
}

pub struct Chunk {
    pos_x: i32,
    pos_y: i32,
    ants:  Vec<Ant>
}

impl Chunk {
    pub fn run(&self){
        for ant in self.ants.iter(){
            ant.run();
        }
    }

}

pub struct World{
    pub chunks: Vec<Box<Chunk>>
}

impl World {

}

fn drawWorld( render_target:&mut RenderWindow, world:& World, sprite: &mut (impl Drawable + Transformable)){
    for chunk in &world.chunks{
        for ant in &chunk.ants{
            sprite.set_position((ant.rel_pos_x as f32 + chunk.pos_x as f32, ant.rel_pos_y as f32 + chunk.pos_y as f32));
            render_target.draw(sprite);
        }
    }
}
fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "Ant path",
        sfml::window::Style::CLOSE,
        &ContextSettings::default()
    );
    let mut clock = Clock::start();
    
    let mut ant = Ant{rel_pos_x:0, rel_pos_y:0, life_time:128, food_taken:128};
    let mut chunk = Chunk{pos_x:100, pos_y:100, ants:Vec::new()};
    let mut world = World{chunks:Vec::new()};
    chunk.ants.push(ant);
    world.chunks.push(Box::new(chunk));

    let mut ant_sprite = CircleShape::default();
    ant_sprite.set_radius(3.);
    ant_sprite.set_outline_thickness(3.);
    ant_sprite.set_outline_color(Color::BLACK);
    ant_sprite.set_fill_color(Color::WHITE);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                sfml::window::Event::Closed => window.close(),
                Event::KeyPressed { code, scan, alt, ctrl, shift, system } =>
                    match code {
                        Key::Escape => window.close(),
                        _ => {}
                    }
                _ => {}
            }
        }
        let elapsed_time = clock.restart().as_seconds();
        window.clear(sfml::graphics::Color::BLACK);
        drawWorld(&mut window, &world, &mut ant_sprite);
        window.display();
    };
}
