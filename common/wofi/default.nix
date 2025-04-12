{ lib, config, pkgs, ... }:

{
  home.packages = 
    let
      wofiPlzFixThisIssue = pkgs.runCommand "wofi-set-term" {} ''
        mkdir -p "$out/bin"
	ln -s "${pkgs.foot}/bin/foot" "$out/bin/xterm"
      '';
    in [
      (lib.hiPrio wofiPlzFixThisIssue)
    ];
  programs.wofi = {
    enable = true;
    settings = {
      allow_images = true;
      hide_scroll = true;
      height = "40%";
      width = "45%";
      image_size = 24;
    };
    style = ''
window {
	background-color: #212121;
	border: 2px solid #252525;
	font-size: 16px;
	font-family: "JetBrainsMono Nerd Font";
}

#scroll {
	border: none;
}

#input {
	border-radius: 0px;
	border: none;
	background-color: #303030;
}

#input:selected {
	border: 1px solid #4c7899;
}

#entry {
	background-color: #212121;
}

#entry:selected {
	background-color: #285577;
	border: 1px solid #4c7899;
	outline: none;
}
    '';
  };
}
