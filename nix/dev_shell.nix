{ name, version, mkShell, rust, nixpkgs-fmt, sqlite, rustfmt, lefthook, diesel-cli }:

mkShell {
  name = "${name}-dev-shell";
  inherit version;

  buildInputs = [
    (rust.override { extensions = [ "rustfmt-preview" "rust-src" "rls-preview" "rust-analysis" ]; })
    nixpkgs-fmt
    rustfmt
    lefthook
    diesel-cli
    sqlite
  ];

  DATABASE_URL = "dev_db.sqlite3";
}
