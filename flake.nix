{
  description = "ccase flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      name = "ccase";
    in
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = name;
          version = (lib.importTOML ./Cargo.toml).package.version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        devShell = pkgs.mkShell {
          name = "ccase";
          buildInputs = with pkgs; [
            just
            watchexec
            cargo
          ];
        };
      }
    );
}
