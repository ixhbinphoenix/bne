{
  description = "TheSchedule development flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
          with pkgs;
          [
            rustup
            openssl
            pkg-config
            gcc
            nodejs
            nodePackages.pnpm
          ];

        shellHook = ''
          rustup default nightly
          rustup component add rust-analyzer
          export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
        '';
      };
    });
}
