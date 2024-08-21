{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }@inputs: 
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in {
      devShells.${system}.default = pkgs.mkShell {
	packages = with pkgs; [
	  openssl
	  (rust-bin.stable.latest.default.override {
	    extensions = [ "rust-src" ];
	  })
    nodejs_22
	];
      };
    };
}
