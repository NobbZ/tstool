{ name, version, mkShell, rust, nixpkgs-fmt, rustfmt, lefthook }:

mkShell {
  name = "${name}-dev-shell";
  inherit version;

  buildInputs = [
    (rust.override { extensions = [ "rust-src" "rls-preview" "rust-analysis" ]; })
    nixpkgs-fmt
    rustfmt
    lefthook
  ];
}
