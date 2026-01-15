{pkgs, ...}: let
  baseImage = pkgs.ociTools.pullImage {
    imageName = "ubuntu";
    tag = "latest";
  };
in
  pkgs.dockerTools.buildImage {
    name = "imphnen-cms-api";

    fromImage = baseImage;

    copyToRoot = pkgs.buildEnv {
      name = "imphnen-cms-api";
      paths = [
        (pkgs.stdenv.mkDerivation {
          name = "imphnen-cms-api";
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
            cp target/release/najm-course-api $out/bin/
          '';
        })
      ];
    };

    config = {
      Cmd = ["/bin/imphnen-cms-api"];
      WorkingDir = "/bin";
    };
  }
