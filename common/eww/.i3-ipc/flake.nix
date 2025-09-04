{
  description = "flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/release-25.05";

  outputs = { nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo
        rustup
        pkg-config
        dbus
      ];
      shellHook = ''
        fish
      '';
    };
  };
}
