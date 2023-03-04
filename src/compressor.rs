use std::path::PathBuf;
use image_compressor::compressor::Compressor;
use image_compressor::Factor;
pub struct Compress {
    pub paths: Vec<PathBuf>,
    pub destination: PathBuf,
    pub quality: f32,
    pub size: f32,
}

impl Compress {
    pub fn new(
        paths: Vec<PathBuf>, 
        destination: PathBuf,
        quality: f32,
        size: f32
    ) -> Self {
        Self { paths, destination, quality, size }
    }
    pub fn compress(&self) {
        println!("compressing {:?} to {:?}", 
             self.paths,
             self.destination
         );
        for path in &self.paths {
            println!("compressing {:?}", path);
            let mut compressor = Compressor::new(
                path, 
                &self.destination,
            );
            compressor.set_factor(Factor::new(self.quality, self.size));
            match compressor.compress_to_jpg(){
                Ok(_) => {},
                Err(e) => println!("Cannot compress file {:?} !: {}", path,e),
            }
        }
    }
}

