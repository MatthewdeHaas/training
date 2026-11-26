{

	description = "Training logger";
	
	inputs = {	
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
				lib = pkgs.lib;
			in
			{
				devShells.default = pkgs.mkShell {
					packages = with pkgs; [
						rustc
						cargo
					];
					RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          shellHook = ''
            echo "Training app dev shell"
            echo "Rust $(rustc --version | cut -d' ' -f2) 🦀"
						echo "Bun $(bun --version) 🥟"
          ''
				};
			};


}
