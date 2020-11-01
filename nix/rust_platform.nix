{ rustChannelOf, makeRustPlatform }:
let
  rustChannel = rustChannelOf {
    date = "2020-10-30";
    channel = "nightly";
    sha256 = "sha256-4NFISQPmGjbz534No4/ZbXaHT5xbjU25WXgj1MTH0TA=";
  };
in
rec {
  inherit (rustChannel) rust rustc cargo;
  rustPlatform = makeRustPlatform {
    rustc = rustChannel.rust;
    cargo = rustChannel.rust;
  };
}
