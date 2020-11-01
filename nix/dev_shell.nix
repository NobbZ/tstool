{ name, version, mkShell, rust, nixpkgs-fmt, rustfmt }:

mkShell {
  name = "${name}-dev-shell";
  inherit version;

  buildInputs = [
    (rust.override { extensions = [ "rust-src" ]; })
    nixpkgs-fmt
    rustfmt
  ];
}