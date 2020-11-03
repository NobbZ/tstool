{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-20.09";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = { url = "github:nmattia/naersk"; inputs.nixpkgs.follows = "nixpkgs"; };

    mozilla = { url = "github:mozilla/nixpkgs-mozilla"; flake = false; };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, mozilla }@inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import mozilla) ];
        };
        naerskLib = pkgs.callPackage naersk {
          cargo = rustTooling.rust;
          rustc = rustTooling.rust;
        };

        rustTooling = pkgs.callPackage ./nix/rust_platform.nix { };

        package = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      in
      {
        devShell = pkgs.callPackage ./nix/dev_shell.nix {
          inherit (package) name version;
          inherit (rustTooling) rust;
        };

        checks.cargo-fmt = pkgs.callPackage ./nix/checks/cargo-fmt.nix {
          inherit (package) name version;
          inherit (rustTooling) rust;
          inherit self;
        };
        checks.nix-fmt = pkgs.callPackage ./nix/checks/nix-fmt.nix {
          inherit (package) name version;
          inherit self;
        };
        checks.build = self.packages.${system}.tstool;

        packages.tstool = naerskLib.buildPackage {
          pname = package.name;
          inherit (package) version;

          src = self;

          buildInputs = [ pkgs.makeWrapper ];

          # cargoSha256 = "sha256-vEoBXYMFhEnMT6U+zZ5Edso5QFDFMTFcRK5Z4a4z1fg=";

          overrideMain = oa: {
            postInstall = ''
              ROCKET_TEMPLATE_DIR=$out/share/tstool/templates
              install -d $ROCKET_TEMPLATE_DIR
              cp -r templates/* $ROCKET_TEMPLATE_DIR

              wrapProgram $out/bin/tstool \
                --set ROCKET_TEMPLATE_DIR $ROCKET_TEMPLATE_DIR
            '';
          };
        };
      });
}
