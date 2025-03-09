{
  description = "ughhh flakes";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    nixos-hardware.url = "github:NixOS/nixos-hardware/master";
  };

  outputs = { self, nixpkgs, home-manager, nixos-hardware, ... }:
    let
      lib = nixpkgs.lib;
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {      
      nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
          inherit system;
          modules =  [
	    ./configuration.nix
            nixos-hardware.nixosModules.lenovo-thinkpad-t480s
	  ];
        };  
    };
}
