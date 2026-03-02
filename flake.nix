{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
            name = "hujte";
            src = ./.;
            buildInputs = [
                pkgs.rustc 
                pkgs.cargo
                pkgs.libxcursor
            ];
            nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];
	    cargoHash = "";
        };

        devShells."x86_64-linux".default= pkgs.mkShell {
            buildInputs = [
                pkgs.rustc 
                pkgs.cargo
                pkgs.libxcursor
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}
