use std::{error::Error, fmt::Display, fs::File, io::{BufReader, BufRead}, path::Path};
use crate::{HitRecord, Hittable, Hittables, brdfs::BRDF, vec_math::{Vec3, cross, dot}};

#[derive(Debug)]
enum WavefrontLine {
    Comment,
    ObjectName(String),
    Vertex(Vec3),
    VertexNormal(Vec3),
    ShadeSmooth(bool),
    Face(usize, usize, usize, usize, usize, usize)
}

#[derive(Debug)]
struct ObjParseError {
    why: &'static str
}

impl Display for ObjParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.why.fmt(f)
    }
}
impl Error for ObjParseError {}

impl WavefrontLine {
    fn from_str(line: &str) -> Result<Self, Box<dyn Error>> {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("#") => Ok(Self::Comment),
            Some("o") => Ok(Self::ObjectName(parts.next().ok_or(ObjParseError{why: "Not enough parts in object name line."})?.to_owned())),
            Some("v") => Ok(Self::Vertex(Vec3::new(parts.next().ok_or(ObjParseError{why: "Not enough parts in vertex name line."})?.parse()?,
                    parts.next().ok_or(ObjParseError{ why: "Not enough parts in vertex line."})?.parse()?,
                    parts.next().ok_or(ObjParseError{ why: "Not enough parts in vertex line."})?.parse()?))),
            Some("vn") => Ok(Self::VertexNormal(Vec3::new(parts.next().ok_or(ObjParseError{why:"Not enough parts in vector normal line."})?.parse()?,
                    parts.next().ok_or(ObjParseError{why:"Not enough parts in vector normal line."})?.parse()?,
                    parts.next().ok_or(ObjParseError{why:"Not enough parts in vector normal line."})?.parse()?,))),
            Some("s") => Ok(Self::ShadeSmooth(match parts.next() {
                Some("0") => Ok(false),
                Some("1") => Ok(true),
                Some(_) => Err( ObjParseError{why: "Invalid smooth shading line. Should be either 0, or 1."}),
                None => Err(ObjParseError{why: "Not enough fields in smooth shading line."}),
            }?)),
            Some("f") => {
                let mut part = parts.next().ok_or(ObjParseError{why: "Not enough parts in face line."})?.split("//");
                let v1: usize = part.next().ok_or(ObjParseError{why: "Not enough numbers in part of face line, each line should have two numbers sepperated by //"})?.parse()?;
                let vn1: usize = part.next().ok_or(ObjParseError{why: "Not enough numbers in part of face line, each line should have two numbers sepperated by //"})?.parse()?;

                let mut part = parts.next().ok_or(ObjParseError{why: "Not enough parts in face line."})?.split("//");
                let v2: usize = part.next().ok_or(ObjParseError{why: "Not enough numbers in part of face line, each line should have two numbers sepperated by //"})?.parse()?;
                let vn2: usize = part.next().ok_or(ObjParseError{why: "Not enough numbers in part of face line, each line should have two numbers sepperated by //"})?.parse()?;

                let mut part = parts.next().ok_or(ObjParseError{why: "Not enough parts in face line."})?.split("//");
                let v3: usize = part.next().ok_or(ObjParseError{why: "Not enough numbers in part of face line, each line should have two numbers sepperated by //"})?.parse()?;
                let vn3: usize = part.next().ok_or(ObjParseError{why: "Not enough numbers in part of face line, each line should have two numbers sepperated by //"})?.parse()?;

                // .obj is 1-indexed subtracting to compensate.
                Ok(Self::Face(v1 - 1, vn1 - 1, v2 - 1, vn2 - 1, v3 - 1, vn3 - 1))
            },
            Some(_) => Err(Box::new(ObjParseError{why:"Unknown line name"})),
            None => Err(Box::new(ObjParseError{why:"Tried to parse empty line"})),
        }
    }
}

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    material: BRDF
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: BRDF) -> Self {
        Self { v0, v1, v2, material }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: crate::ray_math::Ray, ray_t: crate::interval::Interval) -> Option<crate::HitRecord> {
        // I have not tested wether this normal points in the same direction as the face normal or
        // opposite it. This does not matter since this will not implement backface culling, but it
        // might matter in the future.
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let normal = cross(v0v1, v0v2);
        let d = -dot(normal, self.v0);
        let t = ray.intersect_plane(normal, d)?;

        if !ray_t.contains(t) { return None }

        let p = ray.at(t);
        
        let v0p = p - self.v0;
        if dot(normal, cross(v0v1, v0p)) < 0.0 { return None }

        let v1v2 = self.v2 - self.v1;
        let v1p = p - self.v1;
        if dot(normal, cross(v1v2, v1p)) < 0.0 { return None }

        let v2v0 = -v0v2;
        let v2p = p - self.v2;
        if dot(normal, cross(v2v0, v2p)) < 0.0 { return None }

        let front_face = dot(ray.direction(), normal) < 0.0;
        let shading_normal = if front_face {
            normal.normalized()
        } else {
            -normal.normalized()
        };

        Some(HitRecord {
            point: p,
            normal: shading_normal,
            t,
            front_face,
            brdf: self.material.clone(),
        })
    }
}

pub struct TriangleMesh {
    surface: Hittables
}

impl TriangleMesh {
    /// Reads the file from 'path' and parses it as a 3d model.
    /// # Errors
    /// Returns io errors and errors if the .obj file is malformed.
    pub fn try_from_obj_file(path: &Path, material: &BRDF) -> Result<Self, Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        let parsed_file = file.lines().map(|x| WavefrontLine::from_str(&x?));
        let mut vertices = Vec::new();
        let mut triangles = Hittables::new();
        
        for line in parsed_file {
            match line? {
                WavefrontLine::Vertex(v) => {
                    println!("Adding vertex at {:?}", v);
                    vertices.push(v)},
                WavefrontLine::Face(v0, _, v1, _, v2, _) => {println!("constructing triangle between {}, {} and {}", v0, v1, v2); triangles.add(Triangle::new(vertices[v0], vertices[v1], vertices[v2], material.clone()))},
                _ => {},
            }
        }

        Ok(Self{surface: triangles})
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: crate::ray_math::Ray, ray_t: crate::interval::Interval) -> Option<crate::HitRecord> {
        self.surface.hit(ray, ray_t)
    }
}

#[cfg(test)]
mod test {
    use crate::{brdfs::make_lambertian_diffuse_brdf, colors::Color, interval::Interval, ray_math::Ray};

use super::*;
    #[test]
    fn test_ray_triangle_intersection_front() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -0.5), Vec3::new(1.0, 0.0, 0.0));
        let triangle = Triangle::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, 1.0, 1.0), make_lambertian_diffuse_brdf(Color::new(1.0, 1.0, 1.0)));
        let expected_t = 1.0;

        let hit = triangle.hit(ray, Interval::new(0.0, 10.0)).expect("This ray should hit");
        assert!(hit.t - expected_t < f64::EPSILON);
    }

    #[test]
    fn test_ray_triangle_intersection_miss() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -0.5), Vec3::new(-1.0, 0.0, 0.0));
        let triangle = Triangle::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, 1.0, 1.0), make_lambertian_diffuse_brdf(Color::new(1.0, 1.0, 1.0)));

        if let Some(_) = triangle.hit(ray, Interval::new(0.0, 10.0)) {
            panic!("This ray should miss");
        }
    }

    #[test]
    fn test_ray_triangle_intersection_back() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -0.5), Vec3::new(-1.0, 0.0, 0.0));
        let triangle = Triangle::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(-1.0, 1.0, -1.0), Vec3::new(-1.0, 1.0, 1.0), make_lambertian_diffuse_brdf(Color::new(-1.0, 1.0, 1.0)));
        let expected_t = 1.0;

        let hit = triangle.hit(ray, Interval::new(0.0, 10.0)).expect("This ray should hit");
        assert!(hit.t - expected_t < f64::EPSILON);
    }
}
