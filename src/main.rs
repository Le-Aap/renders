use renders::{camera::CameraBuilder, vec_math::Vec3, Hittables, Sphere};

fn main() {
    let mut world = Hittables::new();

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera =
        CameraBuilder::new()
        .set_aspect_ratio(16.0/9.0)
        .set_image_width(1080)
        .to_camera();

    camera.render(&world);
}
