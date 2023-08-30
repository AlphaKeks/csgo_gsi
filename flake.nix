{
	inputs = {
		nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
		utils.url = github:numtide/flake-utils;
	};

	outputs = { nixpkgs, utils, ... }: utils.lib.eachDefaultSystem(system: let
		pkgs = import nixpkgs {
			inherit system;
		};
	in {
		devShell = pkgs.mkShell {
			nativeBuildInputs = with pkgs; [
				rustup
			];

			buildInputs = with pkgs; [
				gnumake
			];

			shellHook = ''
				rustup toolchain install stable
				rustup toolchain install nightly --profile minimal
				rustup +nightly component add rustfmt
				rustup default stable
				rustup update
			'';
		};
	});
}
