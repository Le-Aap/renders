use std::{
    fs, io::{
        prelude::*,
        BufWriter
    }
};
use renders::{ray_math::Ray, vec_math::Vec3};

fn main() {
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let image_height = (f64::from(image_width) / aspect_ratio) as i32;
    let image_height = if image_height < 1 {1} else {image_height};

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Vec3::new(0.0, 0.0, 0.0);
    
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = BufWriter::new(fs::File::create("image.ppm").expect("Error creating file."));

    file.write_all(
        format!("P3\n{image_width} {image_height}\n255\n")
            .as_bytes()
    ).expect("Error while writing to file-buffer");

    for j in 0..image_height {
        print!("\rScanlines remaining: {}        ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel_origin + (f64::from(i) * pixel_delta_u) + (f64::from(j) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let camera_ray = Ray::new(camera_center, ray_direction);

            let color = renders::ray_color(&camera_ray);

            file.write_all(
                color
                    .to_string()
                    .as_bytes()
            ).expect("Error while writing to file-buffer");
        }
    }

    file.flush().expect("Error while executing file-writes");
    println!("\rDone.                           ");
}
