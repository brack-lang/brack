{
  description = "A very basic flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system}; in
      rec {
        packages = flake-utils.lib.flattenTree
          {
            racco = pkgs.nimPackages.buildNimPackage {
              name = "racco";
              src = ./.;
            };
          };
        defaultPackage = packages.racco;
        apps.racco = flake-utils.lib.mkApp { drv = packages.racco; };
        defaultApp = apps.racco;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;  [ nim-unwrapped-2 nimPackages.nimble darwin.apple_sdk.frameworks.Security ];
        };
      }
    );
}