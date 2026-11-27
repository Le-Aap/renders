<!--
SPDX-FileCopyrightText: 2026 Isabelle de Boer <isabelledeboer1@proton.me>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# RenderSDF
Renders started of as, and on the master branch for now still is a Rust 
implementation of the book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

The goal of the project initially was to learn Rust, but now I want to leave
the old and bad code behind! Mostly to be able to apply some of the new skills I
have gained, learn to use some well-loved crates such as Rayon and learn how to
actually build rust with nix tooling.

In addition to me wanting to gain newer technical skills, my interests and goals
for the project have shifted from being a simple monte-carlo triangle renderer
to implementing some newer techniques for pathtracing, and instead of rendering
scenes made up from triangles I want to shift towards rendering signed distance
fields using sphere tracing.

The goals I have currently in my todo are quite a lot so it will likely take a
while but eventually I want to use the (two) render methods built here for v2
of the project and integrate it with the Bevy engine into a real-time
application for SDF based 3D-modeling.

## Todo:
- [x] Set up nix for building, running and testing the project.
- [ ] Build up a solid library of functions needed for sphere-tracing.
- [ ] Build a biased renderer based on ReSTIR.
- [ ] Build an unbiased spectral renderer based on BDPT.

## License
Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

at your option.
