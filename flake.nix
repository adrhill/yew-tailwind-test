{
  description = "WASM Test";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustWasmTarget = "wasm32-unknown-unknown";
        rustToolchain = with fenix.packages.${system};
          combine [
            latest.rustc
            latest.cargo
            targets.${rustWasmTarget}.latest.rust-std
          ];
        rustPlatform = pkgs.makeRustPlatform {
          rustc = rustToolchain;
          cargo = rustToolchain;
        };
        buildInputs = with pkgs; [
          rustToolchain
          trunk
          lld
          nodejs
          nodePackages.npm
          nodePackages.autoprefixer
          nodePackages.postcss
          nodePackages.tailwindcss
          tailwindcss
        ];
      in {
        devShells.default = pkgs.mkShell { nativeBuildInputs = buildInputs; };

        defaultPackage = rustPlatform.buildRustPackage {
          pname = "wasm-test-nix";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = buildInputs;
          cargoLock = { lockFile = ./Cargo.lock; };

          # export HOME=$TMPDIR
          # npm config set strict-ssl false
          buildPhase = ''
            trunk build --public-url "https://adrianhill.de/wasm-test/" --release --dist $out
          '';
        };
      });
}
