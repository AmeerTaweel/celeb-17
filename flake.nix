{
	description = "Celeb-17 Music Player";
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
	inputs.flake-utils.url = "github:numtide/flake-utils";

	outputs = { self, nixpkgs, flake-utils }:
	flake-utils.lib.eachDefaultSystem (system: let
		pkgs = nixpkgs.legacyPackages.${system};
	in {
		devShell = pkgs.mkShell {
			nativeBuildInputs = with pkgs; [
				rustc
				cargo
				yt-dlp
				pkg-config
				alsa-lib
			];
			buildInputs = [ ];
		};
	});
}
