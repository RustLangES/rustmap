{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    workerd = {
      url = "github:getchoo/workerd-docker";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    wrangler = {
      # Use 4.19.1
      url = "github:ryand56/wrangler/1141a859c59e05ceb901d14790f0f75a6c5de3f5";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        pkgs = import nixpkgs { inherit system; };
        fenix = inputs.fenix.packages.${system};
        workerd = inputs.workerd.packages.${system}.workerd;
        wrangler = inputs.wrangler.packages.${system}.wrangler;

        # fenix: rustup replacement for reproducible builds
        toolchain = fenix.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
        };
      in {
        # `nix develop`
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            pkg-config
            git-cliff
            wrangler

            # Rust
            toolchain
            cargo-dist

            # NodeJs
            nodejs_22
            bun
          ];

          MINIFLARE_WORKERD_PATH = "${workerd}/bin/workerd";
        };
      }
    );
}
