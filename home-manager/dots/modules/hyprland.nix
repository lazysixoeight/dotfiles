{config, pkgs, ...}:

{
  wayland.windowManager.hyprland = {
    enable = true;
    plugins = [
      pkgs.hyprlandPlugins.hyprscroller
      pkgs.hyprlandPlugins.hy3
      #pkgs.hyprlandPlugins.hyprbars
    ];
    extraConfig = builtins.readFile ~/.config/home-manager/dots/hypr/Hyprscroller-theme/hyprland.conf;
  }; 
}
