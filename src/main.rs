extern crate sfml;

use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::system::Clock;
use sfml::window::{VideoMode, Style, ContextSettings};

fn main() {
   let mut window = RenderWindow::new(VideoMode::new(800, 600, 32), "SFML window", sfml::window::Style::CLOSE, &ContextSettings::default());
//    let mut clock = Clock::new();

   while window.is_open() {
       while let Some(event) = window.poll_event() {
           match event {
               sfml::window::Event::Closed => window.close(),
               _ => {}
           }
       }

       window.clear(sfml::graphics::Color::BLACK);
       window.display();
   }
}
