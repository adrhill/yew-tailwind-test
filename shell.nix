{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    wasm-pack
    trunk
    lld
    nodejs_23
    tailwindcss
  ];
}
