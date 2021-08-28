with import ./.nix/nixpkgs.nix;
mkShell {
  name = "embedded-shell";
  buildInputs = [
    gnumake
    rustup
    wget
    pkg-config
    alsaLib
    cmake
    python3
    freetype
    expat
    xorg.libX11
  ];
}
