use std::fs;

use webp_to_png::TargetFile;

fn main() {
    let paths = fs::read_dir("./input/").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "webp"
                || extension == "png"
                || extension == "PNG"
                || extension == "jpg"
                || extension == "JPG"
                || extension == "jpeg"
            {
                let img = TargetFile::new(&path.into_os_string().into_string().unwrap()).unwrap();
                img.replace_webp_with_png();
            }
        }
    }
}
