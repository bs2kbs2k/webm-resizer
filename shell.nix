{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    rustc
    pkg-config
    autoconf
    automake
    ffmpeg
    libvpx
    libopus
  ];
}