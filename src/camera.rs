use crate::nalgebra;
use crate::transform;

pub struct Camera {
    aspect_ratio: f32,
    pub transform: transform::Transform
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            transform: transform::Transform::new()
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let projection = nalgebra::Perspective3::new(self.aspect_ratio, 1.0, 0.1, 1024.0);
        let projection_matrix = projection.to_homogeneous();
        let projection_ref: &[[f32; 4]; 4] = projection_matrix.as_ref();

        return *projection_ref;
    }

    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }
}