use renders::{Hittables, Sphere, camera::CameraBuilder, vec_math::Vec3};

fn main() {
    let mut world = Hittables::new();

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = CameraBuilder::new()
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_bounces(50)
        .set_aspect_ratio(16.0/9.0)
        .to_camera();

    camera.render(&world);
}
