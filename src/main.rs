use renders::{brdfs::{self, BRDF}, camera::CameraBuilder, colors::Color, vec_math::Vec3, Hittables, Sphere};

fn main() {
    let mut world = Hittables::new();

    let ground_material: BRDF = brdfs::make_lambertian_diffuse_brdf(Color::new(0.8, 0.8, 0.0));
    let center_material: BRDF  = brdfs::make_lambertian_diffuse_brdf(Color::new(0.1, 0.2, 0.5));
    let left_material: BRDF = brdfs::make_metal_brdf(Color::new(0.8, 0.8, 0.8));
    let right_material: BRDF = brdfs::make_metal_brdf(Color::new(0.8, 0.6, 0.2));

    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, center_material));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left_material));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right_material));

    let camera = CameraBuilder::new()
        .set_aspect_ratio(16.0/9.0)
        .set_image_width(400)
        .set_samples_per_pixel(300)
        .set_nr_threads(8)
        .to_camera();

    camera.render(&std::sync::Arc::new(world));
}
