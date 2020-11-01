{ stdenvNoCC, self, name, version, rustfmt, nixpkgs-fmt, rust }:

stdenvNoCC.mkDerivation {
  pname = "${name}-check";
  inherit version;

  phases = [ "unpackPhase" "buildPhase" ];

  src = self;

  buildInputs = [ rustfmt ];

  buildPhase = ''
    ${nixpkgs-fmt}/bin/nixpkgs-fmt --check **/*.nix *.nix | tee $out
    ${rust}/bin/cargo fmt -- --check | tee $out
  '';
}
