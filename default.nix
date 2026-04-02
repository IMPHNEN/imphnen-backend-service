{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage {
  pname = "imphnen-backend";
  version = (pkgs.lib.importTOML ./imphnen-backend/Cargo.toml).package.version;
  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
  cargoBuildFlags = [
    "--package"
    "imphnen-backend"
    "--bin"
    "api"
  ];
  nativeBuildInputs = [ pkgs.pkg-config ];
  buildInputs = [ pkgs.openssl ];
  doCheck = false;
}
