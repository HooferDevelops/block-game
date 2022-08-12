use std::collections::HashMap;

use glium::Surface;
use glium::glutin;
use glium::glutin::event;
use glium::glutin::event::VirtualKeyCode;

use glium::uniform;

use crate::textures;
use crate::shaders;
use crate::models;
use crate::cube;
use crate::camera;
use crate::meshbuilder;
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
    cursor_locked: MouseState,
    delta_time: f32,
    meshes: HashMap<String, meshbuilder::Mesh>
}

#[derive(PartialEq)]
enum MouseState {
    Unlocked,
    NeedsLocked,
    Locked
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
            cursor_locked: MouseState::Unlocked,
            delta_time: 1.0,
            meshes: HashMap::new()
        }
    }

    // Initialize base data
    pub fn ready(&mut self) {
        self.active_camera.transform.set_position(nalgebra::Point3::new(0.0, 2.0, 0.0));
        self.active_camera.transform.set_rotation(nalgebra::Vector3::new(-1.5, 0.0, 0.0));

        let window = self.display.as_ref().unwrap().gl_window();
        window.window().focus_window();

        self.cursor_locked = MouseState::NeedsLocked;

        let mut mesh = meshbuilder::MeshBuilder::new();

        mesh.set_texture(self.textures.get_texture("texture_atlas", self.display.as_ref().unwrap()).unwrap());
        mesh.set_shader(self.shaders.get_shader_program("basic", &self.display.as_ref().unwrap()).unwrap());

        let mut dirt = cube::Cube::new();

        for (_face_type, face) in dirt.faces.iter_mut() {
            face.set_face_texture_offset(
                (16.0 / 256.0, 16.0 / 256.0),
                (0.0, 1.0)
            );
        }

        let mut grass = cube::Cube::new();

        for (face_type, face) in grass.faces.iter_mut() {
            match face_type {
                cube::Faces::Top => {
                    face.set_face_texture_offset(
                        (16.0 / 256.0, 16.0 / 256.0),
                        (0.0 + (64.0 / 256.0), 1.0)
                    );
                },
                cube::Faces::Bottom => {
                    face.set_face_texture_offset(
                        (16.0 / 256.0, 16.0 / 256.0),
                        (0.0, 1.0)
                    );
                },
                _ => {
                    face.set_face_texture_offset(
                        (16.0 / 256.0, 16.0 / 256.0),
                        (0.0 + (32.0 / 256.0), 1.0)
                    );
                }
            }
        }

        for x in 1..100 {
            for y in 1..100 {
                let mut cube = dirt.clone();

                cube.translate_local(nalgebra::Vector3::new(x as f32, 0.0, y as f32));
                mesh.add_cube(cube);

                let mut cube = grass.clone();
                cube.translate_local(nalgebra::Vector3::new(x as f32, 1.0, y as f32));
                mesh.add_cube(cube);
            }
        }

        self.meshes.insert("flat".to_string(), mesh.build(self.display.as_ref().unwrap()));
    }

    // Update Skybox
    pub fn update_skybox(&mut self) {
        let mut sky = meshbuilder::MeshBuilder::new();
        
        sky.set_texture(self.textures.get_texture("sky", self.display.as_ref().unwrap()).unwrap());
        sky.set_shader(self.shaders.get_shader_program("sky", &self.display.as_ref().unwrap()).unwrap());

        let mut skybox = self.models.get_model("skybox").unwrap();

        skybox.scale_local(1000.0);
        skybox.translate_local(self.active_camera.transform.get_position().coords);

        sky.add_model(skybox);

        self.meshes.insert("sky".to_string(), sky.build(self.display.as_ref().unwrap()));
    }

    // Create information
    pub fn create_window(&mut self, event_loop: &glutin::event_loop::EventLoop<()>, name: &str, width: u32, height: u32, _fullscreen: bool, vsync: bool) {
        let icon_rgba = self.textures.icon_rgba8("icon");
        let icon: Result<glutin::window::Icon, glutin::window::BadIcon> = glutin::window::Icon::from_rgba(icon_rgba, 400, 400);
        
        let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
        .with_title(name)
        .with_window_icon(Some(icon.unwrap()));

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
        self.shaders.load_shader("cloud").unwrap();
        self.shaders.load_shader("sky").unwrap();
        
    }

    // Preload Models
    pub fn load_models(&mut self) {
        self.models.load_model("skybox").unwrap();
    }

    // Preload Textures
    pub fn load_textures(&mut self) {
        self.textures.load_image("sky").unwrap();
        self.textures.load_image("texture_atlas").unwrap();
    }

    // Game tick
    pub fn game_tick(&mut self) -> bool {
        let mut camera_speed: f32 = 0.4;

        match self.active_keys.get(&VirtualKeyCode::LShift) {
            Some(true) => {
                camera_speed /= 4.0;
            }
            _ => {}
        }

        for (key, value) in self.active_keys.iter_mut() {
            if *value {
                match key {
                    VirtualKeyCode::W => {
                        self.active_camera.transform.translate_local(nalgebra::Vector3::new(0.0, 0.0, -camera_speed));
                    }
                    VirtualKeyCode::S => {
                        self.active_camera.transform.translate_local(nalgebra::Vector3::new(0.0, 0.0, camera_speed));
                    }
                    VirtualKeyCode::A => {
                        self.active_camera.transform.translate_local(nalgebra::Vector3::new(-camera_speed, 0.0, 0.0));
                    }
                    VirtualKeyCode::D => {
                        self.active_camera.transform.translate_local(nalgebra::Vector3::new(camera_speed, 0.0, 0.0));
                    }
                    VirtualKeyCode::Q => {
                        self.active_camera.transform.translate_local(nalgebra::Vector3::new(0.0, -camera_speed, 0.0));
                    }
                    VirtualKeyCode::E => {
                        self.active_camera.transform.translate_local(nalgebra::Vector3::new(0.0, camera_speed, 0.0));
                    }
                    VirtualKeyCode::Escape => {
                        return true;
                    }
                    _ => {}
                }
            }
        }

        self.update_skybox();

        return false;
    }

    // Drawing tick
    pub fn draw_tick(&mut self) {
        let mut target = self.display.as_ref().unwrap().draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            max_anisotropy: 1,
            ..Default::default()
        };

        let params = glium::DrawParameters {
            //polygon_mode: glium::draw_parameters::PolygonMode::Line,
            //line_width: Some(1.0),
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //blend: glium::Blend::alpha_blending(),
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let cam_matrix = self.active_camera.transform.get_matrix();
        let cam_persp = self.active_camera.get_perspective();
        let cam_pos = *self.active_camera.transform.get_position().coords.as_ref();

        for (_mesh_name, mesh) in &self.meshes {
            let uniforms = uniform! {
                model: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                view: cam_matrix,
                perspective: cam_persp,
                camera_position: cam_pos,
                tex: glium::uniforms::Sampler(mesh.texture.as_ref().expect("No texture"), behavior)
            };

            target.draw(
                mesh.vertices.as_ref().expect("No vertices"), 
                mesh.indices.as_ref().expect("No indices"), 
                mesh.shader.as_ref().expect("No shader"),
                &uniforms, 
                &params
            ).unwrap();
        }

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
                            match self.cursor_locked {
                                MouseState::Unlocked => {
                                    self.cursor_locked = MouseState::NeedsLocked;
                                },
                                _ => {
                                    self.cursor_locked = MouseState::Unlocked;
                                }
                            }
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

                let (x, y) = (position.x as i32, position.y as i32);
                
                let (wx, wy) = (window_size.width as i32, window_size.height as i32);

                let (x_diff, y_diff) = (x - wx / 2, y - wy / 2);

                // Rotate the camera
                
                if (self.cursor_locked == MouseState::Locked && self.window_focused) {
                    self.active_camera.transform.rotate_local(
                        nalgebra::Vector3::new(-(y_diff as f32) * 0.01, -(x_diff as f32) * 0.01, 0.0)
                    );

                    // Clamp X rotation
                    let mut old_rotation = self.active_camera.transform.get_rotation();
                    old_rotation.x = old_rotation.x.min(1.5).max(-1.5);
                    self.active_camera.transform.set_rotation(old_rotation);

                    self.cursor_locked = MouseState::NeedsLocked;
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

    pub fn center_cursor(&mut self) {
        let window = self.display.as_ref().unwrap().gl_window();
        let (width, height) = self.display.as_ref().unwrap().get_framebuffer_dimensions();

        let x = (width as f32 / 2.0) as i32;
        let y = (height as f32 / 2.0) as i32;
    
        self.display.as_ref().unwrap().gl_window().window().set_cursor_visible(!(self.window_focused && (self.cursor_locked != MouseState::Unlocked)));

        match window.window().set_cursor_position(glutin::dpi::LogicalPosition::new(x, y)) {
            Ok(_) => {
                match window.window().set_cursor_grab(glutin::window::CursorGrabMode::Confined) {
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn window_update(&mut self) {
        if (self.cursor_locked == MouseState::NeedsLocked) {
            self.center_cursor();
            self.cursor_locked = MouseState::Locked;
        }

        let window = self.display.as_ref().unwrap().gl_window();

        if (self.cursor_locked == MouseState::Unlocked) {
            self.display.as_ref().unwrap().gl_window().window().set_cursor_visible(true);
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