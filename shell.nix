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
    gcc
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
  ];
  LD_LIBRARY_PATH = "${xorg.libXcursor}/lib:${xorg.libXrandr}/lib:${xorg.libXi}/lib:${vulkan-headers}/include/vulkan:${vulkan-loader}/lib";
}
