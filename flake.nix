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
                pkgs.cairo 
                pkgs.gtk4 
                pkgs.alsa-lib 
                pkgs.xdotool 
                pkgs.xorg.libXi 
                pkgs.gcc 
                pkgs.libinput.dev 
                pkgs.systemdMinimal 
                pkgs.xorg.libX11.dev 
                pkgs.xorg.libXtst
                pkgs.rustc 
                pkgs.cargo
		pkgs.pipewire
            ];
            nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];
	    cargoHash = "";
        };

        devShells."x86_64-linux".default= pkgs.mkShell {
            buildInputs = [
                pkgs.xorg.libXi
                pkgs.gcc
                pkgs.libinput.dev 
                pkgs.systemdMinimal 
                pkgs.xorg.libX11.dev 
                pkgs.xorg.libXtst
                pkgs.rustc
                pkgs.cargo
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}
