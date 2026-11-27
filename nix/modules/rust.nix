# This file is taken from github:srid/rust-nix-template
# Copyright (c) 2023 Sridhar Ratnakumar
# Licensed under the MIT License.
# See the LICENSES/MIT-rust-nix-template.txt file for the full license text.

{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    packages.default = self'.packages.rendersdf;
  };
}
