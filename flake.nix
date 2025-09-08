{
  description = "My Yew & Tailwind project";

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
        rustPlatform =
          pkgs.makeRustPlatform {
            rustc = rustToolchain;
            cargo = rustToolchain;
          };
        buildInputs = with pkgs; [
            rustToolchain
            wasm-bindgen-cli_0_2_100 # match Cargo.toml
            wasm-pack
            trunk
            lld
            nodejs
            node2nix
            tailwindcss
          ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = buildInputs;
        };
        defaultPackage = rustPlatform.buildRustPackage {
          pname = "yew-tailwind-test-nix";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = with pkgs; [
            rustToolchain
            wasm-bindgen-cli_0_2_100 # match Cargo.toml
            wasm-pack
            trunk
            tailwindcss
            lld
            # pkgs.pkg-config
            # pkgs.openssl
          ];

          cargoLock = { lockFile = ./Cargo.lock; };

          buildPhase = ''
            trunk build --release --dist $out         
          '';
        };
      }
    );
}
