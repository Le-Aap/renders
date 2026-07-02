use std::{fs::File, io::{self, BufRead}, path::Path};

use crate::{Hittable, Hittables, vec_math::Vec3};

#[derive(Debug)]
enum WavefrontLine {
    Comment,
    ObjectName(String),
    Vertex(Vec3),
    VertexNormal(Vec3),
    ShadeSmooth(bool),
    Face(usize, usize, usize, usize, usize, usize)
}

impl WavefrontLine {
    fn from_str(line: String) -> WavefrontLine {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("#") => Self::Comment,
            Some("o") => Self::ObjectName(parts.next().expect("Should have object name after o in file").to_owned()),
            Some("v") => Self::Vertex(Vec3::new(parts.next().expect("Vertex line should contain 3 floating point values.").parse().expect("Vertex line should contain a valid floating point value."),
                    parts.next().expect("Vertex line should contain 3 floating point values.").parse().expect("Vertex line should contain a valid floating point value."),
                    parts.next().expect("Vertex line should contain 3 floating point values.").parse().expect("Vertex line should contain a valid floating point value."))),
            Some("vn") => Self::VertexNormal(Vec3::new(parts.next().expect("Vertex line should contain 3 floating point values.").parse().expect("Vertex line should contain a valid floating point value."),
                    parts.next().expect("Vertex line should contain 3 floating point values.").parse().expect("Vertex line should contain a valid floating point value."),
                    parts.next().expect("Vertex line should contain 3 floating point values.").parse().expect("Vertex line should contain a valid floating point value."))),
            Some("s") => Self::ShadeSmooth(match parts.next() {
                Some("0") => false,
                Some("1") => true,
                Some(_) => panic!("part after s in .obj file must be 0 or 1"),
                None => panic!("empty shade smooth line in .obj file"),
            }),
            Some("f") => {
                let mut part = parts.next().expect("f line should contain 3 fields.").split("//");
                let v1 = part.next().expect("f line should have 2 numbers per part.").parse().expect("This should be an index");
                let vn1 = part.next().expect("f line should have 2 numbers per part.").parse().expect("This should be an index");

                let mut part = parts.next().expect("f line should contain 3 fields.").split("//");
                let v2 = part.next().expect("f line should have 2 numbers per part.").parse().expect("This should be an index");
                let vn2 = part.next().expect("f line should have 2 numbers per part.").parse().expect("This should be an index");

                let mut part = parts.next().expect("f line should contain 3 fields.").split("//");
                let v3 = part.next().expect("f line should have 2 numbers per part.").parse().expect("This should be an index");
                let vn3 = part.next().expect("f line should have 2 numbers per part.").parse().expect("This should be an index");

                Self::Face(v1, vn1, v2, vn2, v3, vn3)
            },
            Some(x) => panic!("Unknown part of .obj: {x}"),
            None => panic!("Malformed line!: {}", line),
        }
    }
}

pub struct TriangleMesh {
    surface: Option<Hittables>
}

impl TriangleMesh {
    pub fn from_obj_file(path: &Path) -> Self {
        let mut file = match File::open(&path) {
            Ok(file) => io::BufReader::new(file),
            Err(why) => panic!("failed to open {}: {}", path.display(), why),
        };
        
        for line in file.lines().map(|x| match x {
            Ok(line) => WavefrontLine::from_str(line),
            Err(why) => panic!("failed to read line from {}: {}", path.display(), why),
        })
        {
            println!("{line:?}");
        }

        todo!()
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: crate::ray_math::Ray, ray_t: crate::interval::Interval) -> Option<crate::HitRecord> {
        match &self.surface {
            Some(hittables) => hittables.hit(ray, ray_t),
            None => None,
        }
    }
}
