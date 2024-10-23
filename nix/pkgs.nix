let
  nixpkgs = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/24.05.tar.gz";
    hash = pkgs.lib.fakeHash;
  };
in
  import nixpkgs {}
