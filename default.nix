# default.nix
#{pkgs ? import <nixpkgs> { } }:
with import <nixpkgs>
{
  overlays = [
    (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
};
let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default); # or `toolchain.minimal`
    rustc = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default); # or `toolchain.minimal`
  };
in
rustPlatform.buildRustPackage rec {
        pname = "fromodoro";
        version = "0.9";
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
        RUSTFLAGS = "-Z allow-features=thread_sleep_until";
}
