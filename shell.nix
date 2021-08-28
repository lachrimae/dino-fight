with import ./.nix/nixpkgs.nix;
mkShell {
  name = "embedded-shell";
  buildInputs = [
    gnumake
    rustup
    wget
  ];
}
