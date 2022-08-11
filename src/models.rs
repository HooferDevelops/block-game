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

#[derive(Clone)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

pub struct Models {
    loaded_models: HashMap<String, Model>
}

impl Model {
    pub fn translate_local(&mut self, direction: nalgebra::Vector3<f32>) {
        for vertex in &mut self.vertices {
            vertex.position = (vertex.position.0 + direction.x, vertex.position.1 + direction.y, vertex.position.2 + direction.z);
        }
    }

    pub fn scale_local(&mut self, scale: f32) {
        for vertex in &mut self.vertices {
            vertex.position = (vertex.position.0 * scale, vertex.position.1 * scale, vertex.position.2 * scale);
        }
    }
}

impl Models {
    pub fn new() -> Models {
        Models {
            loaded_models: HashMap::new()
        }
    }

    pub fn load_model(&mut self, file_name: &str) -> Result<bool, String> {
        let model_name = file_name.to_owned() + ".obj";

        let model_source = MODELS.get_file(model_name.to_owned()).unwrap();
        let loaded_model: obj::Obj<obj::TexturedVertex, u32> = obj::load_obj(model_source.contents()).unwrap();

        let vertices: Vec<Vertex> = loaded_model.vertices.iter().map(|v: &obj::TexturedVertex| {
            Vertex {
                position: (v.position[0], v.position[1], v.position[2]),
                normal: (v.normal[0], v.normal[1], v.normal[2]),
                tex_coords: [v.texture[0], v.texture[1]]
            }
        }).collect();

        self.loaded_models.insert(file_name.to_owned(), Model { vertices: vertices, indices: loaded_model.indices });

        Ok(true)
    }


    pub fn get_model(&mut self, model_name: &str) -> Result<Model, String> {
        match self.loaded_models.get(model_name) {
            Some(model) => {
                Ok(model.clone())
            },
            None => {
                Err("Model not found".to_owned())
            }
        }
    }
}