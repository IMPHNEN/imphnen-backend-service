{ pkgs ? import <nixpkgs> { } }:
let
  swaggerUi = pkgs.fetchurl {
    url = "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.17.14.zip";
    hash = "sha256-SBJE0IEgl7Efuu73n3HZQrFxYX+cn5UU5jrL4T5xzNw=";
  };
in
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
  preBuild = ''
    export SWAGGER_UI_DOWNLOAD_URL="file://${swaggerUi}"
  '';
  doCheck = false;
}
