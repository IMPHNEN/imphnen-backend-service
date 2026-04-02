{ pkgs, ... }:
let
  baseImage = pkgs.ociTools.pullImage {
    imageName = "ubuntu";
    tag = "latest";
  };
in
pkgs.dockerTools.buildImage {
  name = "imphnen-backend-service";

  fromImage = baseImage;

  copyToRoot = pkgs.buildEnv {
    name = "imphnen-backend-service";
    paths = [
      (pkgs.stdenv.mkDerivation {
        name = "imphnen-backend-service";
        src = ./src;

        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.openssl
          pkgs.pkg-config
        ];

        buildPhase = ''
          cargo build --release
        '';

        installPhase = ''
          mkdir -p $out/bin
          cp target/release/imphnen-backend-service $out/bin/
        '';
      })
    ];
  };

  config = {
    Cmd = [ "/bin/imphnen-backend-service" ];
    WorkingDir = "/bin";
  };
}
