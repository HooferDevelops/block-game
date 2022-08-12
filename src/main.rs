#![allow(unused_parens)]

extern crate glium;
extern crate image;
extern crate fixedstep;
extern crate nalgebra;

use glium::glutin;

mod textures;
mod shaders;
mod transform;
mod cube;
mod models;
mod meshbuilder;
mod camera;
mod game;

fn main() {
    print!("1");
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut active_game = game::Game::new();
    active_game.create_window(&event_loop, "┬─┬ ノ( ゜-゜ノ)", 1280, 720, false, true);
    active_game.load_textures();
    active_game.load_shaders();
    active_game.load_models();
    active_game.ready();
    active_game.start_loop(event_loop);
}
