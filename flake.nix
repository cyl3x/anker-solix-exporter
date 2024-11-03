{
  description = "anker-solix-exporter";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      commonArgs = {
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = craneLib.filterCargoSources;
        };

        strictDeps = true;

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      anker-solix-exporter = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });
    in {
      checks = { inherit anker-solix-exporter; };
      packages = {
        inherit anker-solix-exporter;
        default = anker-solix-exporter;
      };

      devShells.default = craneLib.devShell {
        inputsFrom = [ anker-solix-exporter ];

        packages = [ pkgs.rust-analyzer rustToolchain ];

        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        RUST_BACKTRACE = 1;
      };
    });
}
