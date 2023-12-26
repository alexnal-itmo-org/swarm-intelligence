extern crate sfml;

use sfml::{
    graphics::{RenderWindow, RenderTarget, Sprite, Shape, Texture, Image, Drawable, Transformable, Color, CircleShape},
    system::{Clock, Time, Vector2i, Vector2f},
    window::{VideoMode, Style, ContextSettings, Event, Key},
};
use crate::world::init_world;

mod ant;
mod chunk;
mod world;

// fn drawWorld(render_target: &mut RenderWindow, world: &World, sprite: &mut (impl Drawable + Transformable)) {
//     for chunk in &world.chunks {
//         for ant in &chunk.ants {
//             sprite.set_position((ant.rel_pos_x as f32 + chunk.pos_x as f32, ant.rel_pos_y as f32 + chunk.pos_y as f32));
//             render_target.draw(sprite);
//         }
//     }
// }

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "Ant path",
        sfml::window::Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut clock = Clock::start();

    let mut world = init_world(16, 16);

    println!("World.run");
    world.run();
    println!("World.run-end");

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
        // drawWorld(&mut window, &world, &mut ant_sprite);
        window.display();
    };
}
