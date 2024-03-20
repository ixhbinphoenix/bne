{
  description = "TheSchedule development flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system: let
      toolchain = fenix.packages.${system}.latest.toolchain;
      fenixpkgs = fenix.packages.${system};
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      devShells.default = pkgs.mkShell {
        buildInputs =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
          rust = fenixpkgs.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rust-analyzer"
            "rustc"
            "rustfmt"
          ];
        in
          with pkgs;
          [
            rust
            openssl
            pkg-config
            gcc
            nodejs
            nodePackages.pnpm
          ];

        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };

      the-backend-bin = (pkgs.makeRustPlatform {
        cargo = toolchain;
        rustc = toolchain;
      }).buildRustPackage {
        pname = "the-backend";
        version = "0.1.0";

        src = ./backend/.;

        nativeBuildInputs = with pkgs; [
          openssl
          pkg-config
          gcc
        ];

        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

        cargoLock = {
          lockFile = ./backend/Cargo.lock;
          outputHashes = {
            "actix-governor-0.5.0" = "sha256-3i0EZIjjPLzrONKKLZWTXRFIEMqpDYo6oEZWE7jQS/A=";
          };
        };
      };

      containerImage = pkgs.dockerTools.buildImage {
        name = "ghcr.io/ixhbinphoenix/bne";
        tag = "latest";

        created = "now"; # Fuck binary compatibility

        contents = [ pkgs.cacert ./backend/email-templates/. ];

        config = {
          Cmd = [
            "${the-backend-bin}/bin/the-backend"
          ];
          Env = [
            "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          ];
        };
      };

      packages.default = containerImage;
    });
}
