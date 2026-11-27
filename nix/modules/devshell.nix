# This file is adapted from github:srid/rust-nix-template
# Copyright (c) 2023 Sridhar Ratnakumar
# Licensed under the MIT License.
# See the LICENSES/MIT-rust-nix-template.txt file for the full license text.

{ inputs, ... }:
{
  perSystem = { config, self', pkgs, ... }: {
    devShells.default = pkgs.mkShell {
      name = "rust-nix-template-shell";
      inputsFrom = [
        self'.devShells.rust
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];
      packages = with pkgs; [
        nixd
        bacon
      ];
    };
  };
}
