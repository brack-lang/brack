{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, fenix, naersk, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (_: super: let pkgs = fenix.inputs.nixpkgs.legacyPackages.${super.system}; in fenix.overlays.default pkgs pkgs)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        naersk' = pkgs.callPackage naersk {};
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            nil
            pkg-config
            openssl
            glib
            libiconv
            darwin.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
            (fenix.packages.${system}.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            rust-analyzer-nightly
          ];
          RUST_SRC_PATH = "${fenix.packages.${system}.complete.rust-src}/lib/rustlib/src/rust/library";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
        packages.default = naersk'.buildPackage {
          src = ./.;
        };
        apps.${system}.default = {
          type = "app";
          program = "${self.packages.default}/bin/brack";
        };
      }
    );
}
