pub struct Camera {
    aspect_ratio: f32,
    position: [f32; 3],
    rotation: [f32; 3]
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0]
        }
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation;
    }

    // https://github.com/glium/glium/blob/master/examples/support/camera.rs#L37
    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    // https://github.com/glium/glium/blob/master/book/tuto-12-camera.md#the-view-matrix
    pub fn get_view_matrix(&self, up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = self.rotation;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-self.position[0] * s_norm[0] - self.position[1] * s_norm[1] - self.position[2] * s_norm[2],
                 -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
                 -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }


    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }
}