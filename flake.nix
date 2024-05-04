{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        packages.default = naersk-lib.buildPackage ./.;
        devShells.default = with pkgs;
          pkgs.clangStdenv.mkDerivation {
            name = "shell";
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              pkg-config
              
              # libraries for mediatoascii to install
              opencv
              ffmpeg
            ];
            LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
