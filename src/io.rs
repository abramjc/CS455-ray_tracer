use crate::common::VertexFormat;
use crate::image::Image;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write_image_ppm<T: VertexFormat>(filename: &str, image: &Image<T>) {
    let path = Path::new(filename);
    let file = File::create(path).unwrap();

    let mut output_stream = BufWriter::new(file);

    write!(output_stream, "P3\n").unwrap();
    write!(output_stream, "{} {}\n", image.width(), image.height()).unwrap();
    write!(output_stream, "{}\n", 255).unwrap();

    for pixel in image.iter() {
        write!(output_stream, " {} ", pixel).unwrap();
    }

    output_stream.flush().unwrap();
}
