use std::collections::HashMap;

use crate::cube;

pub struct Block {
    pub cubes: Vec<cube::Cube>,
    name: String
}

/*impl Block {
    pub fn build_to_mesh(&self, mesh: &meshbuilder::MeshBuilder, position: nalgebra::Point3<f32>) {

        for cube in self.cubes.iter() {
            let mut cube_copy = cube.clone();

            cube_copy.translate_local(position);
            mesh.add_cube(cube_copy);
        }
    }
}*/

pub struct BlockBuilder {
    cubes: Option<Vec<cube::Cube>>,
    transparent: bool,
    name: String
}

impl BlockBuilder {
    pub fn new() -> BlockBuilder {
        BlockBuilder {
            cubes: None,
            transparent: false,
            name: "none".to_string()
        }
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();

        self
    }

    pub fn set_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;

        self
    }

    pub fn add_cube(mut self, cube: cube::Cube) -> Self {
        if self.cubes.is_none() {
            self.cubes = Some(Vec::new());
        }

        self.cubes.as_mut().unwrap().push(cube);

        self
    }

    pub fn build(&self, add_to: Option<&mut HashMap<String, Block>>) {
        let new_block = Block {
            cubes: self.cubes.as_ref().unwrap().clone(),
            name: self.name.to_string(),
        };

        match add_to {
            Some(blocks) => {
                blocks.insert(self.name.clone(), new_block);
            },
            None => {}
        }
    }
}