use std::{collections::HashMap};
use include_dir::{include_dir, Dir};

static SHADERS: Dir<'_> = include_dir!("shaders");


struct Shader {
    vertex: String,
    fragment: String
}

pub struct Shaders {
    loaded_shaders: HashMap<String, Shader>
}

impl Shaders {
    pub fn new() -> Shaders {
        Shaders {
            loaded_shaders: HashMap::new()
        }
    }

    pub fn load_shader(&mut self, file_name: &str) -> Result<bool, String> {
        let fragment_name = file_name.to_owned() + "-frag.glsl";
        let vertex_name = file_name.to_owned() + "-vert.glsl";

        let fragment_source = SHADERS.get_file(fragment_name.to_owned()).unwrap(); //File::open("shaders/".to_owned() + &fragment_name).unwrap();
        let vertex_source = SHADERS.get_file(vertex_name.to_owned()).unwrap();

        let fragment_source_string = fragment_source.contents_utf8().unwrap();
        let vertex_source_string = vertex_source.contents_utf8().unwrap();
        
        self.loaded_shaders.insert(file_name.to_owned(), Shader { vertex: vertex_source_string.to_string(), fragment: fragment_source_string.to_string() });

        Ok(true)
    }

    pub fn get_shader_program(&mut self, shader_name: &str, display: &glium::Display) -> Result<glium::Program, String> {
        match self.loaded_shaders.get(shader_name) {
            Some(shader) => {
                Ok(
                    glium::Program::from_source(
                        display, 
                        shader.vertex.as_str(), 
                        shader.fragment.as_str(), 
                        None
                    ).unwrap()
                )
            },
            None => {
                Err("Shader not found".to_owned())
            }
        }
    }
}