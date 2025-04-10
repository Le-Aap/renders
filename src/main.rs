use std::{
    fs,
    io::{
        prelude::*,
        BufWriter
    }
};
// use::renders;

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
            let r:f64 = f64::from(j) / f64::from(image_width - 1);
            let g:f64 = f64::from(i) / f64::from(image_height - 1);
            let b:f64 = 0.0;

            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let r:u8 = (255.999 * r).floor() as u8;
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let g:u8 = (255.999 * g).floor() as u8;
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let b:u8 = (255.999 * b).floor() as u8;

            file.write_all(
                format!("{r} {g} {b}\n")
                    .as_bytes()
            ).expect("Error while writing to file-buffer");
        }
    }

    file.flush().expect("Error while executing file-writes");
    println!("\rDone.                           ");
}
