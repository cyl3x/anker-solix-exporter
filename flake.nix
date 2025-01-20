{
  description = "Flake anker-solix-exporter";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      systems = [ "aarch64-linux" "x86_64-linux" ];

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        rust-project = {
          crates."anker-solix-exporter".crane.args = {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
          };
        };

        overlayAttrs = { inherit (self'.packages) anker-solix-exporter; };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self'.devShells.rust ];

          RUST_LOG = "info";
          RUST_BACKTRACE = "full";
        };
      };
    };
}
