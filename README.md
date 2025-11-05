# RendeRS
![Render produced by RendeRS](https://github.com/Le-Aap/renders/blob/master/image.png)
RendeRS is primarily a rust implementation of the raytracer described in the book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

So far implemented from the book is chapter 1 - 10.
In addition to this I have implemented multithreading.

## To run with Nix
To run RendeRS when using the Nix package manager on Linux/WSL:
Clone RendeRS and open the project root in a shell. In the shell run `nix develop` to install all of the requirements needed to build RendeRS. 
The versions installed are the same used for testing and so are guaranteed to work.
After running `nix develop` run `cargo run -r` to compile and run RendeRS.

## To run without Nix
To run RendeRS when not using the Nix package manager, make sure you have a version of [the rust toolchain installed](https://rust-lang.org/tools/install/) installed.

Clone RendeRS and open the project root in a shell, then run `cargo run -r` to compile and run RendeRS.

## Todo:
My goal is to extend this implementation with:
- The rest of the content of the book.
- Support for rendering triangle meshes using a BVH acceleration structure.
- Output to more image types.
- A version of the universal Disney BRDF for compatibility with 3d modeling programs.
- Importing scene descriptor files from 3d modeling programs.
- A web ui for a more friendly interface.
