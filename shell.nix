{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  buildInputs = [
    cmake
    pkg-config
    fontconfig
    # wayland
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
    libGL

    openssl # for wasm-bindgen-cli
  ];

  # WINIT_UNIX_BACKEND = "wayland";

  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}
