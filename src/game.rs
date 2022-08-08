use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use glium::Surface;
use glium::glutin;
use glium::glutin::event;
use glium::glutin::event::VirtualKeyCode;
use glium::glutin::event_loop::EventLoop;
use glium::uniform;
use spin_sleep::LoopHelper;

use crate::textures;
use crate::shaders;
use crate::models;
use crate::camera;

pub struct Game {
    shaders: shaders::Shaders,
    models: models::Models,
    textures: textures::Textures,
    active_camera: camera::Camera,
    display: Option<glium::Display>,
    game_loop: fixedstep::FixedStep,
    active_keys: HashMap<VirtualKeyCode, bool>,
    last_game_tick: std::time::Instant
}

impl Game {
    pub fn new() -> Game {
        Game {
            shaders: shaders::Shaders::new(),
            models: models::Models::new(),
            textures: textures::Textures::new(),
            active_camera: camera::Camera::new(),
            display: None,
            game_loop: fixedstep::FixedStep::start(60.0),
            active_keys: HashMap::new(),
            last_game_tick: std::time::Instant::now()
        }
    }

    // Initialize base data
    pub fn ready(&mut self) {
        self.active_camera.set_position([3.0,3.0,-1.0]);
        self.active_camera.set_rotation([-0.5,-0.5,0.5]);
    }

    // Create information
    pub fn create_window(&mut self, event_loop: &glutin::event_loop::EventLoop<()>, name: &str, width: u32, height: u32, fullscreen: bool, vsync: bool) {
        let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
        .with_title(name);

        let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(vsync)
        .with_multisampling(4);

        let new_display = glium::Display::new(
            window_builder, 
            context_builder, 
            event_loop
        ).unwrap();

        self.active_camera.update_aspect_ratio(width, height);

        self.display = Some(new_display);
    }

    // Preload Shaders
    pub fn load_shaders(&mut self) {
        self.shaders.load_shader("basic").unwrap();
    }

    // Preload Models
    pub fn load_models(&mut self) {
        self.models.load_model("cube", self.shaders.get_shader_program("basic", self.display.as_ref().unwrap()).unwrap(), self.display.as_ref().unwrap()).unwrap();
        self.models.load_model("grass_cube", self.shaders.get_shader_program("basic", self.display.as_ref().unwrap()).unwrap(), self.display.as_ref().unwrap()).unwrap();
    
    }

    // Preload Textures
    pub fn load_textures(&mut self) {
        self.textures.load_image("test_texture").unwrap();
        self.textures.load_image("grass_texture").unwrap();
    }

    // Game tick
    pub fn game_tick(&mut self) {
        match self.active_keys.get(&VirtualKeyCode::W) {
            Some(true) => {
                let mut old_position = self.active_camera.get_position();
                old_position[0] += 0.1;
                self.active_camera.set_position(old_position);
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::S) {
            Some(true) => {
                let mut old_position = self.active_camera.get_position();
                old_position[0] -= 0.1;
                self.active_camera.set_position(old_position);
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::A) {
            Some(true) => {
                let mut old_position = self.active_camera.get_position();
                old_position[2] += 0.1;
                self.active_camera.set_position(old_position);
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::D) {
            Some(true) => {
                let mut old_position = self.active_camera.get_position();
                old_position[2] -= 0.1;
                self.active_camera.set_position(old_position);
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::Q) {
            Some(true) => {
                let mut old_position = self.active_camera.get_position();
                old_position[1] -= 0.1;
                self.active_camera.set_position(old_position);
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::E) {
            Some(true) => {
                let mut old_position = self.active_camera.get_position();
                old_position[1] += 0.1;
                self.active_camera.set_position(old_position);
            },
            _ => {}
        }

        println!("game tick!");
    }

    // Drawing tick
    pub fn draw_tick(&mut self) {
        let model: &models::Model = self.models.get_model("grass_cube").unwrap();
        let texture = self.textures.get_texture("grass_texture", self.display.as_ref().unwrap()).unwrap();

        let mut target = self.display.as_ref().unwrap().draw();
        target.clear_color_and_depth((0.15, 0.15, 0.15, 1.0), 1.0);
        
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        let uniforms = uniform! {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 2.0, 1.0f32]
            ],
            tex: glium::uniforms::Sampler(&texture, behavior),

            view: self.active_camera.get_view_matrix(&[0.0, 1.0, 0.0]),
            perspective: self.active_camera.get_perspective()
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(
            &model.vertices, 
            &model.indices,
            &model.shader, 
            &uniforms, 
            &params
        ).unwrap();

        target.finish().unwrap();
    }

    pub fn keyboard_input(&mut self, event: &glium::glutin::event::WindowEvent<'_>) {
        let key_input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };

        let pressed = key_input.state == glutin::event::ElementState::Pressed;
        match key_input.virtual_keycode {
            Some(key) => {
                self.active_keys.insert(key, pressed);
            },
            None => return,
        };
    }

    pub fn process_events(&mut self, event: event::Event<()>, ) -> bool {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    return true;
                    //*control_flow = glutin::event_loop::ControlFlow::Exit;
                    //return;
                },
                glutin::event::WindowEvent::Resized(physical_size) => {
                    self.active_camera.update_aspect_ratio(physical_size.width, physical_size.height);
                },
                sub_event => self.keyboard_input(&sub_event),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => {
                }
                glutin::event::StartCause::Init => {
                
                }
                _ => (),
            },
            _ => (),
        }

        return false;
    }
    
    // Main Loop
    pub fn start_loop(mut self, event_loop: glutin::event_loop::EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::MainEventsCleared => {
                    while self.game_loop.update() {
                        self.game_tick();
                    }
                    
                    let _delta = self.game_loop.render_delta();
                    self.draw_tick();
                },
                _ => ()
            }

            let should_quit = self.process_events(event);

            if (should_quit) {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        });
    }
}