{
  description = "IMPHNEN Backend Service Nix Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    supportedSystems = ["x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux"];
    pkgsFor = system:
      import nixpkgs {
        inherit system;
        config = {
          allowUnfree = true;
        };
      };
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
  in {
    packages = forAllSystems (system: {
      default = (pkgsFor system).callPackage ./default.nix {};
    });
    devShells = forAllSystems (system: {
      default = (pkgsFor system).callPackage ./shell.nix {};
    });
    dockerImages = forAllSystems (system: {
      tryOutApi = (pkgsFor system).callPackage ./docker.nix {};
    });
  };
}
