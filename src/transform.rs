pub struct Transform {
    position: nalgebra::Point3<f32>,
    rotation: nalgebra::Vector3<f32>
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: nalgebra::Point3::new(0.0, 0.0, 0.0),
            rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_position(&self) -> nalgebra::Point3<f32> {
        return self.position;
    }

    pub fn set_position(&mut self, position: nalgebra::Point3<f32>) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> nalgebra::Vector3<f32> {
        return self.rotation;
    }

    pub fn set_rotation(&mut self, rotation: nalgebra::Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn rotate_local(&mut self, angle: nalgebra::Vector3<f32>) {
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

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let mut mat4 = nalgebra::Matrix4::new_translation(&self.position.coords);
        //mat4 *= nalgebra::Vector3::new(1.0, 1.0, 1.0); // Scaling
        mat4 *= self.rotation_matrix();
        mat4 *= nalgebra::Rotation::face_towards(
            &nalgebra::Vector3::new(0.0, 0.0, 1.0), 
            &nalgebra::Vector3::new(0.0, 1.0, 0.0)
        ).to_homogeneous();
        
        let mat4_result = mat4.try_inverse().unwrap();

        let view_projection_ref: &[[f32; 4]; 4] = mat4_result.as_ref();
        
        return *view_projection_ref;
    }
}