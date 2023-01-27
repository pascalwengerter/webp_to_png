use image;
use image::imageops;
use std::fs;
use std::path::Path;

pub struct TargetFile {
    pub name: String,
    pub output: String,
}

impl TargetFile {
    ///Creates a struct for an image, turning the source into an output path.

    pub fn new(source: &String) -> Result<Self, &'static str> {
        Ok(Self {
            name: source.clone(),
            output: source
                .clone()
                .replace("input", "output")
                .replace("webp", "png")
                .replace("jpg", "png")
                .replace("jpeg", "png")
                .replace("JPEG", "png")
                .replace("JPG", "png"),
        })
    }

    ///Saves a copy of the image as the desired output (PNG with 512px width/height) and deletes the original file.

    pub fn replace_webp_with_png(&self) {
        let file = Path::new(&self.name);
        let target = Path::new(&self.output);
        let img = image::open(file).expect("Could not open the file!");
        img.thumbnail(512, 512)
            .resize(512, 512, imageops::Lanczos3)
            .save(target)
            .expect("Could not save file to new file!");
        println!("Converted {} to {}!", &self.name, &self.output);
        fs::remove_file(file).unwrap();
    }
}
