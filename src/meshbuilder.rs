use crate::shaders;
use crate::models;

pub struct Mesh {
    pub vertices: glium::vertex::VertexBuffer<models::Vertex>,
    pub indices: glium::index::IndexBuffer<u32>
}

pub struct MeshBuilder {
    pub vertices: Vec<models::Vertex>,
    pub indices: Vec<u32>
}

impl MeshBuilder {
    pub fn new(shaders: &shaders::Shaders) -> MeshBuilder {
        MeshBuilder {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }
    
    pub fn add_vertex(&mut self, vertex: models::Vertex) {
        //self.vertices.push(vertex);
    }
    
    pub fn add_index(&mut self, index: u32) {
        //self.indices.push(index);
    }

    pub fn add_model(&mut self, mut model: models::Model) {
        let i = self.vertices.len() as u32;

        for index in &mut model.indices {
            *index += i;
        }

        self.vertices.extend_from_slice(&model.vertices);
        self.indices.extend_from_slice(&model.indices);
    }
    
    pub fn build(&self, display: &glium::Display) -> Mesh {
        let vertices = glium::vertex::VertexBuffer::new(
            display, 
            &self.vertices
        ).unwrap();

        let indices: glium::index::IndexBuffer<u32> = glium::index::IndexBuffer::new(
            display, 
            glium::index::PrimitiveType::TrianglesList, 
            &self.indices
        ).unwrap();

        Mesh {
            vertices,
            indices
        }
    }
}