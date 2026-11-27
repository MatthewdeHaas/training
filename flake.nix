{
  description = "Training logger";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "training-logger";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        postInstall = ''
          ln -s $out/bin/food.rs $out/bin/food
          ln -s $out/bin/train.rs $out/bin/train
        '';
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ self.packages.${system}.default ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
				packages = with pkgs; [
					sqlite-interactive
					clippy
					rustfmt
				];
        shellHook = ''
          echo "Training log"
          echo "Rust $(rustc --version | cut -d' ' -f2) 🦀"
        '';
      };
    };
}
