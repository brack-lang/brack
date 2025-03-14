{
  runCommand,
  rust-bin,
}:
let
  toolchain = rust-bin.stable.latest.default;
in
runCommand "cargo-fmt-check" {
  buildInputs = [
    toolchain
  ];
  src = ../.;
} ''
  mkdir -p $out
  cargo fmt --all --check --manifest-path $src/Cargo.toml
''
