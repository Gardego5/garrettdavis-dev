{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    cargo2nix.inputs.rust-overlay.follows = "rust-overlay";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, cargo2nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        rustVersion = "latest";
        rustChannel = "nightly";
        packageFun = import ./Cargo.nix;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
          config.allowUnfreePredicate = pkg:
            builtins.elem (pkgs.lib.getName pkg) [
              "terraform"
            ];
        };

        pkgsCross = import nixpkgs {
          inherit system;
          crossSystem.config = "aarch64-unknown-linux-musl";
          overlays = [ cargo2nix.overlays.default ];
        };

        rustpkgs = pkgs.rustBuilder.makePackageSet {
          inherit
            rustVersion
            rustChannel
            packageFun;

          extraRustComponents = [ "clippy" ];
        };

        rustpkgs-lambda = pkgsCross.rustBuilder.makePackageSet {
          inherit
            rustVersion
            rustChannel
            packageFun;

          target = "aarch64-unknown-linux-musl";
        };
      in {
        devShell = rustpkgs.workspaceShell {
          packages = with pkgs; [ terraform just tailwindcss ];
        };

        packages.default = (rustpkgs-lambda.workspace.garrettdavis-dev { }).bin;
      }
    );
}
