{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs"; # avoids duplicating nixpkgs
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { overlays = [ rust-overlay.overlays.default ]; inherit system; };
      lib = nixpkgs.lib;
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      rustPlatform = pkgs.makeRustPlatform {
        # inherit (rustToolchain) cargo rustc;
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
    in
    {

      devShells.${system}.default = pkgs.mkShell {

        buildInputs = (with pkgs; [
          xorg.libxcb
          libxml2
        ]);

        nativeBuildInputs = [
          rustToolchain
          rustPlatform.bindgenHook
        ];

      };

      formatter.${system} = pkgs.nixpkgs-fmt;
    };
}
