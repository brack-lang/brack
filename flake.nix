{
  description = "A bracket-based lightweight markup language that extends commands with WebAssembly";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
            ];
          };

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              nil
              rust-bin.stable.latest.default
              rust-analyzer
            ];
          };

          packages.default = pkgs.callPackage ./nix/build.nix {
            doCheck = false;
          };

          formatter = pkgs.nixfmt-rfc-style;

          checks = {
            cargo-test = pkgs.callPackage ./nix/build.nix { };
            cargo-fmt-check = pkgs.callPackage ./nix/cargo-fmt-check.nix { };
            nixfmt-check = pkgs.callPackage ./nix/nixfmt-check.nix { };
            clippy-check = pkgs.callPackage ./nix/clippy-check.nix { };
            actionlint-check = pkgs.callPackage ./nix/actionlint-check.nix { };
          };

          apps.default = {
            type = "app";
            program = "${self'.packages.default}/bin/brack";
          };

          apps.brack-release = {
            type = "app";
            program = "${self'.packages.default}/bin/brack-release";
          };
        };
    };
}
