# Nix shell for rust development
# Run "nix-shell" in this directory to have access to rust/cargo/etc.

{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    # nativeBuildInputs = with pkgs; [];
    # ^^ Only needed for developing Nix or NixOS

    buildInputs = with pkgs; [
        # common build inputs
        direnv

        # project specific
        rustup
        glib

    ];

    env = {};

}