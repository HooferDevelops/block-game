use std::{collections::HashMap};
use image::ImageBuffer;
use include_dir::{include_dir, Dir};


static TEXTURES: Dir<'_> = include_dir!("textures");

struct Texture {
    rgba8: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    dimensions: (u32, u32)
}

pub struct Textures {
    loaded_textures: HashMap<String, Texture>
}

impl Textures {
    pub fn new() -> Textures {
        Textures {
            loaded_textures: HashMap::new()
        }
    }

    pub fn load_image(&mut self, file_name: &str) -> Result<bool, String> {
        let image_name = file_name.to_owned() + ".png";

        let image_source = TEXTURES.get_file(image_name.to_owned()).unwrap(); //File::open("shaders/".to_owned() + &fragment_name).unwrap();

        let image_bytes = image_source.contents();

        let loaded_image = image::load_from_memory(&image_bytes).unwrap().to_rgba8();
        let dimensions = loaded_image.dimensions();

        self.loaded_textures.insert(file_name.to_owned(), Texture {
            rgba8: loaded_image,
            dimensions: dimensions
            //Image: glium::texture::RawImage2d::from_raw_rgba_reversed(&loaded_image.into_raw(), dimensions)
        });

        Ok(true)
    }

    pub fn icon_rgba8(&self, file_name: &str) -> Vec<u8> {
        let image_name = file_name.to_owned() + ".png";

        let image_source = TEXTURES.get_file(image_name.to_owned()).unwrap(); //File::open("shaders/".to_owned() + &fragment_name).unwrap();

        let image_bytes = image_source.contents();

        let loaded_image = image::load_from_memory(&image_bytes).unwrap().to_rgba8();
        
        return loaded_image.into_raw();
    }

    pub fn get_texture(&mut self, texture_name: &str, display: &glium::Display) -> Result<glium::texture::SrgbTexture2d, String> {
        match self.loaded_textures.get(texture_name) {
            Some(texture) => {
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&texture.rgba8.as_raw(), texture.dimensions);
                let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
                
                Ok(
                    texture
                )
            },
            None => {
                Err("Texture not found".to_owned())
            }
        }
    }
}