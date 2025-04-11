use std::{
    fs,
    io::{
        prelude::*,
        BufWriter
    }
};
use renders::vec_math::Vec3;
use renders::vec_math;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut file = BufWriter::new(fs::File::create("image.ppm").expect("Error creating file."));

    file.write_all(
        format!("P3\n{image_width} {image_height}\n255\n")
            .as_bytes()
    ).expect("Error while writing to file-buffer");

    for i in 0..image_width {
        print!("\rScanlines remaining: {}        ", image_height - i);
        for j in 0..image_height {
            let x_fade = f64::from(j) / f64::from(image_width - 1);
            
            let color = Vec3::new(
                x_fade,
                x_fade,
                x_fade
            );

            file.write_all(
                color
                    .format_as_color()
                    .as_bytes()
            ).expect("Error while writing to file-buffer");
        }
    }

    file.flush().expect("Error while executing file-writes");
    println!("\rDone.                           ");
}
