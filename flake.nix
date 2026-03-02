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
                pkgs.libX11.dev
            ];
            nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];
	    cargoHash = "sha256-l9sG+TZ2pa0iKLshURwWjsuy20nY2/4djy5GmODld68=";
        };

        devShells."x86_64-linux".default= pkgs.mkShell {
            buildInputs = [
                pkgs.libX11.dev 
                pkgs.libxcursor
                pkgs.rustc
                pkgs.cargo
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}
