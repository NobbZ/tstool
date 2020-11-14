{ rustChannelOf, makeRustPlatform }:
let
  rustChannel = rustChannelOf {
    date = "2020-11-14";
    channel = "nightly";
    sha256 = "sha256-1LPuJj+z1fRwzzEUDct2mtKwxv7nO9lspU7QFYigiOw=";
  };
in
rec {
  inherit (rustChannel) rust rustc cargo;
  rustPlatform = makeRustPlatform {
    rustc = rustChannel.rust;
    cargo = rustChannel.rust;
  };
}
