{ self, naersk, pkgs, name, version, rustPlatform }:
let
  naerskLib = pkgs.callPackage naersk {
    inherit (rustPlatform.rust) cargo rustc;
  };
in
naerskLib.buildPackage {
  pname = name;
  inherit version;

  src = self;

  buildInputs = [ pkgs.makeWrapper ];

  overrideMain = oa: {
    postInstall = ''
      ROCKET_TEMPLATE_DIR=$out/share/tstool/templates
      install -d $ROCKET_TEMPLATE_DIR
      cp -r templates/* $ROCKET_TEMPLATE_DIR

      wrapProgram $out/bin/tstool \
        --set ROCKET_TEMPLATE_DIR $ROCKET_TEMPLATE_DIR
    '';
  };
}
