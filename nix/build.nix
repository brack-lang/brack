{
  makeRustPlatform,
  rust-bin,
  pkgs-config,
  openssl,
  doCheck ? true,
}:
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

  buildInputs = [
    openssl
    openssl.dev
  ];

  nativeBuildInputs = [ pkgs-config ];

  inherit doCheck;
}
