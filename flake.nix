{
  description = "Full-stack development environment with Rust backend and Next.js frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-darwin"];

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        overlay = final: prev: let
          nodejs = prev.nodejs_22;
          pnpm = prev.pnpm_10.override {inherit nodejs;};
        in {
          inherit nodejs pnpm;
        };
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay) overlay];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer" "clippy"];
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            rustfmt
            nodejs
            pnpm
            sqldef
            pkg-config
            openssl
          ];
        };

        treefmt = {
          projectRootFile = "flake.nix";
          programs.rustfmt.enable = true;
          programs.prettier.enable = true;
          settings.formatter.prettier.excludes = ["**/pnpm-lock.yaml"];
        };
      };
    };
}
