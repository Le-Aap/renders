use renders::{brdfs::{self}, camera::CameraBuilder, colors::Color, vec_math::Vec3, Hittables, Sphere};

fn main() {
    let mut world = Hittables::new();

    let material_ground = brdfs::make_lambertian_diffuse_brdf(Color::new(0.8, 0.8, 0.0));
    let material_center = brdfs::make_lambertian_diffuse_brdf(Color::new(0.1, 0.2, 0.5));
    let material_left = brdfs::make_glass_brdf(1.50, Color::new(1.0, 1.0, 1.0));
    let material_bubble = brdfs::make_glass_brdf(1.00/1.50, Color::new(1.0, 1.0, 1.0));
    let material_right = brdfs::make_metal_brdf(Color::new(0.8, 0.6, 0.2));

    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let camera = CameraBuilder::new()
        .set_nr_threads(15)
        .set_aspect_ratio(16.0/9.0)
        .set_image_width(2560)
        .set_samples_per_pixel(500)
        .set_max_bounces(50)
        .set_vfov(20.0)
        .set_look_from(Vec3::new(-2.0, 2.0, 1.0))
        .set_look_at(Vec3::new(0.0, 0.0, -1.0))
        .set_camera_up(Vec3::new(0.0, 1.0, 0.0))
        .to_camera();

    camera.render(&std::sync::Arc::new(world));
}
