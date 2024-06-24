{
  description = "A bracket-based lightweight markup language that extends commands with WebAssembly";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    crate2nix,
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
        buildInputsForBuild = with pkgs;
          [
            openssl
            openssl.dev
          ]
          ++ pkgs.lib.optional pkgs.stdenv.isDarwin [
            darwin.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        nativeBuildInputsForBuild = with pkgs; [pkg-config];
        customBuildRustCrateForPkgs = pkgs:
          pkgs.buildRustCrate.override {
            defaultCrateOverrides =
              pkgs.defaultCrateOverrides
              // {
                brack = attrs: {
                  buildInputs = buildInputsForBuild;
                  nativeBuildInputs = nativeBuildInputsForBuild;
                };
              };
          };
        generatedBuild = pkgs.callPackage ./Cargo.nix {
          buildRustCrateForPkgs = customBuildRustCrateForPkgs;
        };
      in rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            buildInputsForBuild
            ++ nativeBuildInputsForBuild
            ++ [
              alejandra
              nil
              toolchain
              rust-analyzer
            ];
        };
        checks = {
          brack = generatedBuild.workspaceMembers.brack.build.override {
            runTests = true;
          };
        };
        packages.brack = generatedBuild.workspaceMembers."brack".build;
        packages.default = packages.brack;
        apps.${system}.default = {
          type = "app";
          program = "${self.packages.default}/bin/brack";
        };
      }
    );
}
