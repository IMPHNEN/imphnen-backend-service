{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  rustDeps = pkgs.callPackage ./Cargo.nix {};
  packageEntry = rustDeps.workspaceMembers.${manifest.name};
  deps = packageEntry.build.cargoDeps or null;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoDeps = deps;
    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
    nativeBuildInputs = [pkgs.openssl pkgs.pkg-config];
    buildInputs = [pkgs.openssl];
  }
