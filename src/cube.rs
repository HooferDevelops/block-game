extern crate serde_json;
extern crate obj;

use crate::models;
use std::{collections::HashMap};

#[derive(Clone)]
pub struct Cube {
    pub faces: HashMap<Faces, Face>
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Faces {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom
}


#[derive(Clone)]
pub struct Face {
    pub vertices: Vec<models::Vertex>,
    pub indices: Vec<u32>
}

impl Face {
    pub fn set_face_texture_offset(&mut self, scale: (f32, f32), offset: (f32, f32)) {
        for vertex in &mut self.vertices {
            vertex.tex_coords = [vertex.tex_coords[0] * scale.0 + offset.0, vertex.tex_coords[1] * scale.0 + offset.1];
        }
    }
}

impl Cube {
    pub fn new() -> Cube {
        let mut cube_faces: HashMap<Faces, Face> = HashMap::new();
            cube_faces.insert(Faces::Front, 
                Face {
                    vertices: vec![
                        models::Vertex {
                            position: (-0.5, -0.5, -0.5),
                            normal: (0.0, 0.0, -1.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, -0.5),
                            normal: (0.0, 0.0, -1.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, -0.5),
                            normal: (0.0, 0.0, -1.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, -0.5),
                            normal: (0.0, 0.0, -1.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (-0.5, 0.5, -0.5),
                            normal: (0.0, 0.0, -1.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, -0.5),
                            normal: (0.0, 0.0, -1.0),
                            tex_coords: [1.0, 1.0]
                        }
                    ],

                    indices: vec![
                        2, 1, 0,
                        0, 4, 2,
                    ]
                }
            );

            cube_faces.insert(Faces::Back, 
                Face {
                    vertices: vec![
                        models::Vertex {
                            position: (-0.5, -0.5, 0.5),
                            normal: (0.0, 0.0, 1.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, 0.5),
                            normal: (0.0, 0.0, 1.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, 0.5),
                            normal: (0.0, 0.0, 1.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, 0.5),
                            normal: (0.0, 0.0, 1.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (-0.5, 0.5, 0.5),
                            normal: (0.0, 0.0, 1.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, 0.5),
                            normal: (0.0, 0.0, 1.0),
                            tex_coords: [0.0, 1.0]
                        }
                    ],

                    indices: vec![
                        0, 1, 2,
                        2, 4, 0,
                    ]
                }
            );

            cube_faces.insert(Faces::Right, 
                Face {
                    vertices: vec![
                        models::Vertex {
                            position: (0.5, 0.5, 0.5),
                            normal: (1.0, 0.0, 0.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, -0.5),
                            normal: (1.0, 0.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, -0.5),
                            normal: (1.0, 0.0, 0.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, -0.5),
                            normal: (1.0, 0.0, 0.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, 0.5),
                            normal: (1.0, 0.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, 0.5),
                            normal: (1.0, 0.0, 0.0),
                            tex_coords: [1.0, 0.0]
                        }
                    ],

                    indices: vec![
                        2, 1, 0,
                        0, 4, 2,
                    ]
                }
            );
        
            cube_faces.insert(Faces::Left, 
                Face {
                    vertices: vec![
                        models::Vertex {
                            position: (-0.5, 0.5, 0.5),
                            normal: (-1.0, 0.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (-0.5, 0.5, -0.5),
                            normal: (-1.0, 0.0, 0.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, -0.5),
                            normal: (-1.0, 0.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, -0.5),
                            normal: (-1.0, 0.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, 0.5),
                            normal: (-1.0, 0.0, 0.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, 0.5, 0.5),
                            normal: (-1.0, 0.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        }
                    ],

                    indices: vec![
                        0, 1, 2,
                        2, 4, 0,
                    ]
                }
            );

            cube_faces.insert(Faces::Top, 
                Face {
                    vertices: vec![
                        models::Vertex {
                            position: (-0.5, 0.5, 0.5),
                            normal: (0.0, 1.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, 0.5),
                            normal: (0.0, 1.0, 0.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, -0.5),
                            normal: (0.0, 1.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, 0.5, -0.5),
                            normal: (0.0, 1.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, 0.5, -0.5),
                            normal: (0.0, 1.0, 0.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, 0.5, 0.5),
                            normal: (0.0, 1.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        }
                    ],

                    indices: vec![
                        0, 1, 2,
                        2, 4, 0,
                    ]
                }
            );

            cube_faces.insert(Faces::Bottom, 
                Face {
                    vertices: vec![
                        models::Vertex {
                            position: (-0.5, -0.5, 0.5),
                            normal: (0.0, -1.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, 0.5),
                            normal: (0.0, -1.0, 0.0),
                            tex_coords: [1.0, 0.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, -0.5),
                            normal: (0.0, -1.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (0.5, -0.5, -0.5),
                            normal: (0.0, -1.0, 0.0),
                            tex_coords: [1.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, -0.5),
                            normal: (0.0, -1.0, 0.0),
                            tex_coords: [0.0, 1.0]
                        },
                        models::Vertex {
                            position: (-0.5, -0.5, 0.5),
                            normal: (0.0, -1.0, 0.0),
                            tex_coords: [0.0, 0.0]
                        }
                    ],

                    indices: vec![
                        2, 1, 0,
                        0, 4, 2,
                    ]
                }
            );


        Cube {
            faces: cube_faces
        }
    }

    pub fn translate_local(&mut self, direction: nalgebra::Point3<f32>) {
        for face in self.faces.values_mut() {
            for vertex in face.vertices.iter_mut() {
                vertex.position = (vertex.position.0 + direction.x, vertex.position.1 + direction.y, vertex.position.2 + direction.z);
            }
        }
    }

    pub fn scale_local(&mut self, scale: f32) {
        for face in self.faces.values_mut() {
            for vertex in face.vertices.iter_mut() {
                vertex.position = (vertex.position.0 * scale, vertex.position.1 * scale, vertex.position.2 * scale);
            }
        }
    }
}
