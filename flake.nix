{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            alejandra
            nil
            toolchain
            rust-analyzer
          ];
        };
        packages.default = rustPlatform.buildRustPackage {
          src = ./.;
          copyLibs = true;
          name = "brack";
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildInputs = with pkgs;
            [pkgconfig]
            ++ pkgs.lib.optional pkgs.stdenv.isDarwin [
              darwin.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];
        };
        apps.${system}.default = {
          type = "app";
          program = "${self.packages.default}/bin/brack";
        };
      }
    );
}
