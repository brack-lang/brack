{ makeRustPlatform, rust-bin, doCheck ? true }:
let
  toolchain = rust-bin.stable.latest.default;
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
in
rustPlatform.buildRustPackage {
  pname = "brack";
  version = "0.2.0";

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  inherit doCheck;
}
