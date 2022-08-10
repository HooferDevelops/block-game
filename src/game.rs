use std::collections::HashMap;

use glium::Surface;
use glium::glutin;
use glium::glutin::event;
use glium::glutin::event::VirtualKeyCode;
use glium::uniform;

use crate::textures;
use crate::shaders;
use crate::models;
use crate::camera;
use crate::nalgebra;

pub struct Game {
    shaders: shaders::Shaders,
    models: models::Models,
    textures: textures::Textures,
    active_camera: camera::Camera,
    display: Option<glium::Display>,
    game_loop: fixedstep::FixedStep,
    active_keys: HashMap<VirtualKeyCode, bool>,
    active_mouse_buttons: HashMap<glutin::event::MouseButton, bool>,
    window_focused: bool,
    cursor_locked: bool,
    delta_time: f32
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
            active_mouse_buttons: HashMap::new(),
            window_focused: true,
            cursor_locked: true,
            delta_time: 1.0
        }
    }

    // Initialize base data
    pub fn ready(&mut self) {
        self.active_camera.set_position([0.0,0.0,0.0]);
        //self.active_camera.set_rotation([0.0,0.0,0.0]);

        let window = self.display.as_ref().unwrap().gl_window();
        window.window().focus_window();

        self.cursor_locked = true;
    }

    // Create information
    pub fn create_window(&mut self, event_loop: &glutin::event_loop::EventLoop<()>, name: &str, width: u32, height: u32, _fullscreen: bool, vsync: bool) {
        let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
        .with_title(name);

        let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(vsync)
        .with_multisampling(16);

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
    pub fn game_tick(&mut self) -> bool {
        match self.active_keys.get(&VirtualKeyCode::W) {
            Some(true) => {
                self.active_camera.translate_local(nalgebra::Vector3::new(0.0, 0.0, 0.1));
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::S) {
            Some(true) => {
                self.active_camera.translate_local(nalgebra::Vector3::new(0.0, 0.0, -0.1));
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::A) {
            Some(true) => {
                self.active_camera.translate_local(nalgebra::Vector3::new(0.1, 0.0, 0.0));
            },
            _ => {}
        }

        match self.active_keys.get(&VirtualKeyCode::D) {
            Some(true) => {
                self.active_camera.translate_local(nalgebra::Vector3::new(-0.1, 0.0, 0.0));
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

        match self.active_keys.get(&VirtualKeyCode::Escape) {
            Some(true) => {
                return true;
            },
            _ => {}
        }

        return false;
    }

    // Drawing tick
    pub fn draw_tick(&mut self) {
        let model: &models::Model = self.models.get_model("grass_cube").unwrap();
        let texture = self.textures.get_texture("grass_texture", self.display.as_ref().unwrap()).unwrap();

        let mut target = self.display.as_ref().unwrap().draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        
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
            perspective: self.active_camera.get_perspective(),
            camera_position: self.active_camera.get_position()
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
        match *event {
            glutin::event::WindowEvent::MouseInput { device_id: _, state, button, .. } => {
                let pressed = state == glutin::event::ElementState::Pressed;

                match button {
                    glutin::event::MouseButton::Left => {
                        self.active_mouse_buttons.insert(glutin::event::MouseButton::Left, pressed);
                    },
                    glutin::event::MouseButton::Right => {
                        self.active_mouse_buttons.insert(glutin::event::MouseButton::Right, pressed);
                    },
                    _ => {}
                }
            },
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                let pressed = input.state == glutin::event::ElementState::Pressed;
                match input.virtual_keycode {
                    Some(key) => {
                        if (pressed && key == VirtualKeyCode::LAlt) {
                            self.cursor_locked = !self.cursor_locked;
                        }

                        self.active_keys.insert(key, pressed);
                    },
                    _ => {}
                }
            },
            glutin::event::WindowEvent::CursorMoved { device_id: _, position, .. } => {
                // Get the direction of the mouse movement
                let window = self.display.as_ref().unwrap().gl_window();

                let window_size = window.window().inner_size();

                let (x, y) = (position.x as f32, position.y as f32);
                let (wx, wy) = (window_size.width as f32, window_size.height as f32);

                let (x_diff, y_diff) = (x - wx / 2.0, y - wy / 2.0);

                // Rotate the camera

                if (self.cursor_locked && self.window_focused) {
                    self.active_camera.rotate(nalgebra::Vector3::new(y_diff * 0.01 * self.delta_time, -x_diff * 0.01 * self.delta_time, 0.0));
                }
            },
            _ => return
        }
    }

    pub fn process_events(&mut self, event: event::Event<()>) -> bool {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    return true;
                },
                glutin::event::WindowEvent::Resized(physical_size) => {
                    self.active_camera.update_aspect_ratio(physical_size.width, physical_size.height);
                },
                glutin::event::WindowEvent::Focused(focused) => {
                    if !focused {
                        self.active_keys.clear();
                    }

                    self.window_focused = focused;
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

    pub fn window_update(&mut self) {
        let window = self.display.as_ref().unwrap().gl_window();
        let (width, height) = self.display.as_ref().unwrap().get_framebuffer_dimensions();

        let x = (width as f32 / 2.0) as i32;
        let y = (height as f32 / 2.0) as i32;
    
        self.display.as_ref().unwrap().gl_window().window().set_cursor_visible(!(self.window_focused && self.cursor_locked));

        if (self.window_focused && self.cursor_locked) {
            match window.window().set_cursor_position(glutin::dpi::LogicalPosition::new(x, y)) {
                Ok(_) => {
                    match window.window().set_cursor_grab(glutin::window::CursorGrabMode::Confined) {
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            match window.window().set_cursor_grab(glutin::window::CursorGrabMode::None) {
                _ => {}
            }
        }
    }
    
    // Main Loop
    pub fn start_loop(mut self, event_loop: glutin::event_loop::EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::MainEventsCleared => {
                    self.window_update();

                    while self.game_loop.update() {
                        let game_exit = self.game_tick();
                        
                        if (game_exit) {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                        }
                    }
                    
                    self.delta_time = self.game_loop.render_delta() as f32;
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