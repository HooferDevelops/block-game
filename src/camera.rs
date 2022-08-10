use std::ops::Mul;

use crate::nalgebra;

pub struct Camera {
    aspect_ratio: f32,
    position: nalgebra::Point3<f32>,
    rotation: nalgebra::Vector3<f32>
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            position: nalgebra::Point3::new(0.0, 0.0, 0.0),
            rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_position(&self) -> [f32; 3] {
        *self.position.coords.as_ref()
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = nalgebra::Point3::new(position[0], position[1], position[2]);
    }

    pub fn rotate(&mut self, angle: nalgebra::Vector3<f32>) {
        self.rotation += angle;
    }

    pub fn translate_local(&mut self, direction: nalgebra::Vector3<f32>) {
        let translation = nalgebra::Matrix4::new_translation(&direction);
        
        let rotation = self.rotation_matrix();
        let inverse_rotation = rotation.try_inverse().unwrap();

        self.position = rotation.transform_point(&(translation * inverse_rotation).transform_point(&self.position));
    }

    pub fn rotation_matrix(&self) -> nalgebra::Matrix4<f32> {
        nalgebra::Matrix4::new_rotation(nalgebra::Vector3::new(0.0, self.rotation.y, 0.0)) *
        nalgebra::Matrix4::new_rotation(nalgebra::Vector3::new(self.rotation.x, 0.0, 0.0)) *
        nalgebra::Matrix4::new_rotation(nalgebra::Vector3::new(0.0, 0.0, self.rotation.z))
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let projection = nalgebra::Perspective3::new(self.aspect_ratio, 1.0, 0.1, 1024.0);
        let projection_matrix = projection.to_homogeneous();
        let projection_ref: &[[f32; 4]; 4] = projection_matrix.as_ref();

        return *projection_ref;
    }

    pub fn get_view_matrix(&self, _up: &[f32; 3]) -> [[f32; 4]; 4] {
        let mut mat4 = nalgebra::Matrix4::new_translation(&self.position.coords);
        //mat4 *= nalgebra::Vector3::new(1.0, 1.0, 1.0); // Scaling
        mat4 *= self.rotation_matrix();
        mat4 *= nalgebra::Rotation::face_towards(
            &nalgebra::Vector3::new(0.0, 0.0, -1.0), 
            &nalgebra::Vector3::new(0.0, 1.0, 0.0)
        ).to_homogeneous();
        
        let mat4_result = mat4.try_inverse().unwrap();


        let view_projection_ref: &[[f32; 4]; 4] = mat4_result.as_ref();
        
        return *view_projection_ref;
    }


    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }
}