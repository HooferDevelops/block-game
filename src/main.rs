
extern crate glium;
extern crate image;
extern crate fixedstep;

use glium::glutin;

mod textures;
mod shaders;
mod models;
mod camera;
mod game;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut active_game = game::Game::new();
    active_game.create_window(&event_loop, "┬─┬ ノ( ゜-゜ノ)", 1280, 720, false, false);
    active_game.load_textures();
    active_game.load_shaders();
    active_game.load_models();
    active_game.ready();
    active_game.start_loop(event_loop);
}
