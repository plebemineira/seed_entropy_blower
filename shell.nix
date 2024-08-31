{ pkgs ? import <nixpkgs> {} }:

let
  lib = pkgs.lib;
  isLinux = pkgs.stdenv.isLinux;
  isDarwin = pkgs.stdenv.isDarwin;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.libiconv
    pkgs.pkg-config
    pkgs.clang
    pkgs.cmake
  ] ++ lib.optional isDarwin pkgs.darwin.apple_sdk.frameworks.AudioUnit
    ++ lib.optional isLinux pkgs.alsaLib;

  shellHook = ''
    export LIBRARY_PATH="$LIBRARY_PATH:${pkgs.libiconv}/lib"
    ''
    + lib.optionalString isDarwin ''
    export CPAL_USE_COREAUDIO=1
    ''
    + lib.optionalString isLinux ''
    export CPAL_USE_ALSA=1
    '';
}
