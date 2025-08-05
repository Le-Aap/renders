use renders::{brdfs::{self, BRDF}, camera::CameraBuilder, colors::Color, vec_math::Vec3, Hittables, Sphere};

fn main() {
    let mut world = Hittables::new();

    let diffuse_grey_material: BRDF = brdfs::make_lambertian_diffuse_brdf(Color::new(0.6, 0.6, 0.6));
    let diffuse_red_material: BRDF  = brdfs::make_lambertian_diffuse_brdf(Color::new(0.9, 0.3, 0.3));

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, diffuse_grey_material.clone()));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, diffuse_red_material.clone()));

    let camera = CameraBuilder::new()
        .to_camera();

    camera.render(&world);
}
