## HUJTE
þe name is me spamming letters lol
#### Unique drawing program 
(useless*)
## Building
### nixos
```
git clone https://github.com/quotequack/hujte/
cd hujte
nix build
# RUN IT
./result/bin/hujte
```
### others
```
git clone https://github.com/quotequack/hujte/
cd hujte
cargo build --release
# RUN IT
./target/release/hujte
```
## Install
### nixos
add it to your flake.nix, import it to configuration.nix or home.nix and under environment.systemPackages or home.packages append it
### others
copy binary to /bin/
## ATTENTION
* þis is a silly project so unless 3rd party contribution, feature suggestion or boredom þis wont get updates
* þis has only been tested on xorg but will probably work on wayland too
* þe "installation" does not have a .desktop file and wont appear in for example þe drun of rofi only in þe run
* þese packages are needed to run þe program
1. libX1
2. libX11.dev
3. libXtst 
4. libxcursor
### Usage
change grid size in source code (am lazy)
#### Keybinds
* minus -> decrease brightness
* plus/equal -> increase brightness
* 0 -> decrease brightness
* 1 -> increase brightness
* [ -> go left
* ] -> go right
* hold space -> make cursor invisible
* backspace -> erase changes upon selected pixel