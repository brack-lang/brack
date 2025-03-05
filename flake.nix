{
  description = "A bracket-based lightweight markup language that extends commands with WebAssembly";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        buildInputsForBuild =
          with pkgs;
          [
            openssl
            openssl.dev
          ]
          ++ pkgs.lib.optional pkgs.stdenv.isDarwin [
            darwin.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        nativeBuildInputsForBuild = with pkgs; [ pkg-config ];
        customBuildRustCrateForPkgs =
          pkgs:
          pkgs.buildRustCrate.override {
            defaultCrateOverrides = pkgs.defaultCrateOverrides // {
              brack = attrs: {
                buildInputs = buildInputsForBuild;
                nativeBuildInputs = nativeBuildInputsForBuild;
              };
              brack-project-manager = attrs: {
                buildInputs = buildInputsForBuild;
                nativeBuildInputs = nativeBuildInputsForBuild;
              };
            };
          };
        generatedBuild = pkgs.callPackage ./Cargo.nix {
          buildRustCrateForPkgs = customBuildRustCrateForPkgs;
        };
        workspaceMemberNames = builtins.attrNames generatedBuild.workspaceMembers;
      in
      rec {
        devShells.default = pkgs.mkShell {
          buildInputs =
            with pkgs;
            buildInputsForBuild
            ++ nativeBuildInputsForBuild
            ++ [
              nixfmt-rfc-style
              nil
              toolchain
              rust-analyzer
              crate2nix
              gh
              shellcheck
            ];
        };
        checks = builtins.listToAttrs (
          map (name: {
            name = name;
            value = generatedBuild.workspaceMembers.${name}.build.override { runTests = true; };
          }) workspaceMemberNames
        );
        packages.cargo-fmt =
          pkgs.runCommand "cargo-fmt-check"
            {
              buildInputs = with pkgs; [
                cargo
                rustfmt
              ];
              src = ./.;
            }
            ''
              mkdir -p $out
              cargo fmt --all --check --manifest-path $src/Cargo.toml
            '';
        packages.nixfmt-rfc-style =
          pkgs.runCommand "nixfmt-rfc-style-check"
            {
              buildInputs = with pkgs; [ nixfmt-rfc-style ];
              src = ./.;
            }
            ''
              mkdir -p $out
              find . -name '*.nix' ! -name 'Cargo.nix' -exec nixfmt-rfc-style --check {} +
            '';
        packages.brack = generatedBuild.workspaceMembers."brack".build;
        packages.default = packages.brack;
        apps.${system}.default = {
          type = "app";
          program = "${self.packages.default}/bin/brack";
        };
      }
    );
}
