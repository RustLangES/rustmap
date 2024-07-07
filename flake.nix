{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        fenix = inputs.fenix.packages;
        # fenix: rustup replacement for reproducible builds
        toolchain = fenix.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-Ngiz76YP4HTY75GGdH2P+APE/DEIx2R/Dn+BwwOyzZU=";
        };
      in {
        # `nix develop`
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            toolchain
            pkg-config
          ];
        };
      }
    );
}
