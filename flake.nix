{
  description = "TheSchedule development flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = github:nix-community/fenix;
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      overlays = [ fenix.overlays.default ];
    in {
      devShells.default = pkgs.mkShell {
        buildInputs =
        let
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
          with pkgs;
          with fenix.packages.${system};
          [
            openssl
            pkg-config
            gcc
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer-nightly
            nodejs
            nodePackages.pnpm
          ];

        shellHook = ''
          export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
        '';
      };
    });
}
