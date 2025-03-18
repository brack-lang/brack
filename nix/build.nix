{
  lib,
  makeRustPlatform,
  rust-bin,
  pkg-config,
  openssl,
  doCheck ? true,
}:
let
  toolchain = rust-bin.stable.latest.default;
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
  version = lib.strings.trim (builtins.readFile ../VERSION);
in
rustPlatform.buildRustPackage {
  pname = "brack";
  inherit version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  buildInputs = [
    openssl
    openssl.dev
  ];

  nativeBuildInputs = [ pkg-config ];

  inherit doCheck;

  APP_VERSION = version;
}
