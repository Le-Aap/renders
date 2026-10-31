# This file is adapted from github:srid/rust-nix-template
# Copyright (c) 2023 Sridhar Ratnakumar
# Licensed under the MIT License.
# See the LICENSES/MIT-rust-nix-template.txt file for the full license text.

{ inputs, ... }:
{
  imports = [
    (inputs.git-hooks + /flake-module.nix)
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    pre-commit.settings = {
      hooks = {
        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
        clippy.enable = true;
        clippy.settings.denyWarnings = true;
      };
    };
  };
}
