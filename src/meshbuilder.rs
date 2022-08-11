use crate::shaders;
use crate::models;

pub struct Mesh {
    pub vertices: Option<glium::vertex::VertexBuffer<models::Vertex>>,
    pub indices: Option<glium::index::IndexBuffer<u32>>,
    pub shader: Option<glium::Program>,
    pub texture: Option<glium::texture::SrgbTexture2d>
}

pub struct MeshBuilder {
    pub vertices: Vec<models::Vertex>,
    pub indices: Vec<u32>,
    pub mesh: Mesh
}

impl Mesh {
    pub fn set_vertices(&mut self, vertices: glium::vertex::VertexBuffer<models::Vertex>) {
        self.vertices = Some(vertices);
    }

    pub fn set_indices(&mut self, indices: glium::index::IndexBuffer<u32>) {
        self.indices = Some(indices);
    }

    pub fn set_shader(&mut self, shader: glium::Program) {
        self.shader = Some(shader);
    }

    pub fn set_texture(&mut self, texture: glium::texture::SrgbTexture2d) {
        self.texture = Some(texture);
    }
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            vertices: Vec::new(),
            indices: Vec::new(),
            mesh: Mesh { vertices: Option::None, indices: Option::None, shader: Option::None, texture: Option::None }
        }
    }
    
    pub fn add_vertex(&mut self, vertex: models::Vertex) {
        //self.vertices.push(vertex);
    }
    
    pub fn add_index(&mut self, index: u32) {
        //self.indices.push(index);
    }

    pub fn set_shader(&mut self, shader: glium::Program) {
        self.mesh.set_shader(shader);
    }

    pub fn set_texture(&mut self, texture: glium::texture::SrgbTexture2d) {
        self.mesh.set_texture(texture);
    }

    pub fn add_model(&mut self, mut model: models::Model) {
        let i = self.vertices.len() as u32;

        for index in &mut model.indices {
            *index += i;
        }

        self.vertices.extend_from_slice(&model.vertices);
        self.indices.extend_from_slice(&model.indices);
    }
    
    pub fn build(mut self, display: &glium::Display) -> Mesh {
        let vertices = glium::vertex::VertexBuffer::new(
            display, 
            &self.vertices
        ).unwrap();

        let indices: glium::index::IndexBuffer<u32> = glium::index::IndexBuffer::new(
            display, 
            glium::index::PrimitiveType::TrianglesList, 
            &self.indices
        ).unwrap();

        self.mesh.set_vertices(vertices);
        self.mesh.set_indices(indices);

        return self.mesh;
    }
}