{
  description = "Advent of Code Rust Environment with oxalica/rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSystem = nixpkgs.lib.genAttrs supportedSystems;
  in {
    devShells = forEachSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Define the Rust toolchain.
        # You can easily swap "stable" for "nightly" or a specific version like "1.75.0"
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };
      in {
        default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.aoc-cli
            pkgs.cargo-show-asm
          ];

          # Pointing the LSP directly to the overlay's source path ensures
          # go-to-definition and autocomplete work seamlessly in your editor.
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "🎄 Advent of Code Rust shell loaded! (via oxalica/rust-overlay)"
            rustc --version
          '';
        };
      }
    );
  };
}
