extern crate serde_json;
extern crate obj;

use std::{collections::HashMap};
use include_dir::{include_dir, Dir};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
    pub tex_coords: [f32; 2]
}

glium::implement_vertex!(Vertex, position, normal, tex_coords);

static MODELS: Dir<'_> = include_dir!("models");

pub struct Model {
    pub vertices: glium::vertex::VertexBuffer<Vertex>,
    pub indices: glium::index::IndexBuffer<u32>,
    pub shader: glium::Program
}

pub struct Models {
    loaded_models: HashMap<String, Model>
}

impl Models {
    pub fn new() -> Models {
        Models {
            loaded_models: HashMap::new()
        }
    }

    pub fn load_model(&mut self, file_name: &str, shader: glium::Program, display: &glium::Display) -> Result<bool, String> {
        let model_name = file_name.to_owned() + ".obj";

        let model_source = MODELS.get_file(model_name.to_owned()).unwrap();
        let loaded_model: obj::Obj<obj::TexturedVertex, u32> = obj::load_obj(model_source.contents()).unwrap();

        let indices: glium::index::IndexBuffer<u32> = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &loaded_model.indices).unwrap();


        let vertices: Vec<Vertex> = loaded_model.vertices.iter().map(|v: &obj::TexturedVertex| {
            Vertex {
                position: (v.position[0], v.position[1], v.position[2]),
                normal: (v.normal[0], v.normal[1], v.normal[2]),
                tex_coords: [v.texture[0], v.texture[1]]
            }
        }).collect();
        
        let vertex_buffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();

        self.loaded_models.insert(file_name.to_owned(), Model { vertices: vertex_buffer, shader: shader, indices: indices });

        Ok(true)
    }

    pub fn get_model(&mut self, model_name: &str) -> Result<&Model, String> {
        match self.loaded_models.get(model_name) {
            Some(model) => {
                Ok(model)
            },
            None => {
                Err("Model not found".to_owned())
            }
        }
    }
}