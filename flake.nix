{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-20.09";
    flake-utils.url = "github:numtide/flake-utils";

    mozilla = { url = "github:mozilla/nixpkgs-mozilla"; flake = false; };
  };

  outputs = { self, nixpkgs, flake-utils, mozilla }@inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import mozilla) ];
        };

        rustTooling = (pkgs.callPackage ./nix/rust_platform.nix { });

        package = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      in
      {
        devShell = pkgs.callPackage ./nix/dev_shell.nix {
          inherit (package) name version;
          inherit (rustTooling) rust;
        };

        checks.formatting = pkgs.stdenvNoCC.mkDerivation {
          name = "${package.name}-check";
          inherit (package) version;

          src = self;

          buildInputs = [ pkgs.rustfmt ];

          buildPhase = ''
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check **/*.nix *.nix
            ${rustTooling.rust}/bin/cargo fmt -- --check
          '';
        };

        packages.tstool = rustTooling.rustPlatform.buildRustPackage {
          pname = package.name;
          inherit (package) version;

          src = self;

          buildInputs = [ pkgs.makeWrapper ];

          cargoSha256 = "sha256-NNefxeYxAnprm1sfuSXrlZudj51sbbISdiLB20nEIXU=";

          postInstall = ''
            wrapProgram $out/bin/tstool \
              --add-flags "--prefix ${placeholder "out"}/usr"
          '';
        };
      });
}
