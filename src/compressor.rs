use std::path::PathBuf;
use image_compressor::compressor::Compressor;
use image_compressor::Factor;
pub struct Compress {
    pub paths: Vec<PathBuf>,
    pub destination: PathBuf,
}

impl Compress {
    pub fn new(paths: Vec<PathBuf>, destination: PathBuf) -> Self {
        Self { paths, destination }
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
            compressor.set_factor(Factor::new(90.0, 1.0));
            match compressor.compress_to_jpg(){
                Ok(_) => {},
                Err(e) => println!("Cannot compress file {:?} !: {}", path,e),
            }
        }
    }
}

