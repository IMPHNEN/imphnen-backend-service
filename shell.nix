{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    crate2nix
    clippy
    postgresql

    (writeScriptBin "helpme" ''
      __usage="
      👋 Welcome to IMPHNEN CMS API development environment. 🚀
      If you see this message, it means your are inside the Nix shell ❄️.

      [Info]===============================================================>

      Rustc Version: v${rustc.version}
      Rustup Version: v${rustup.version}
      Cargo Version: v${cargo.version}

      Command available:
        - start:            start project in production 🛹 ( need to run build first )
        - build:            build project for production
        - dev:              start project in development
        - start-docker:     start project in docker container ( compose )
        - build-docker:     build project for docker container
        - helpme:           show this messages

      Repository:
        - https://github.com/IMPHNEN/imphnen-cms-api
      [Info]===============================================================>
      "
      echo "$__usage"
    '')

    (writeScriptBin "dev" ''
      cargo watch -x run
    '')

    (writeScriptBin "start" ''
      echo "Starting project in production mode..."
      echo "Najm Course API started on port $PORT 🛹..."
      ./result/bin/imphnen-cms-api
    '')

    (writeScriptBin "build" ''
      echo "Building project..."
      crate2nix generate
      nix build -f Cargo.nix
      echo "Now you can start the project with the command 'start'"
    '')

    (writeScriptBin "start-docker" ''
      echo "Starting project in docker container..."
      docker compose up -d
    '')

    (writeScriptBin "build-docker" ''
      echo "Building project with docker..."
      docker build -t imphnen-api:latest .
      echo "Project built successfully."
      echo "Now you can start the project with the command 'start-docker'"
    '')
  ];

  shellHook = ''
    helpme
    if [ -f .env ]; then
       echo "Loading .env file..."
       export $(cat .env | xargs)
       echo "Successfully applied .env file."
     else
       echo ".env file not found."
     fi
  '';
}
