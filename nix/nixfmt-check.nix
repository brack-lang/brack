{
  runCommand,
  nixfmt-rfc-style,
}:
runCommand "nixfmt-check"
  {
    buildInputs = [
      nixfmt-rfc-style
    ];
    src = ../.;
  }
  ''
    mkdir -p $out
    find . -name '*.nix' -exec nixfmt-rfc-style --check {} +
  ''
