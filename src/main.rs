extern crate piston;
extern crate opengl_graphics;
extern crate glutin_window;

use opengl_graphics::glyph_cache::GlyphCache;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub mod game;
pub mod models;
pub mod music;

pub const W_HEIGHT: f64 = 600.0;
pub const W_WIDTH: f64 = 900.0;

/// constructs a window and starts game instance 
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("berzerk",
                                                 [W_WIDTH as u32, W_HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .expect("Error creating window");

    let mut gl = GlGraphics::new(opengl);
    let mut g = game::Game::new(W_WIDTH, W_HEIGHT);
    let mut glyph_cache = GlyphCache::new("assets/Amatic-Bold.ttf").expect("Error getting fonts");
    g.run(&mut window, &mut gl, &mut glyph_cache);
}


#[cfg(test)] 
mod berzerk_test {
    use super::*;

    #[test]
    fn test_window_creation() {
        let opengl = OpenGL::V3_2;
        let _window: Window = WindowSettings::new("create test",
                                             [500, 500])
        .exit_on_esc(true)
        .build()
        .expect("Error creating window"); 
        let _gl = GlGraphics::new(opengl);    
    }

}