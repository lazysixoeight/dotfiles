{ config, pkgs, ... }:

{
  imports = [
   ./hardware-configuration.nix
  ];

  nix.settings.experimental-features = [
    "nix-command"
    "flakes"
  ];

  programs.zsh.enable = true;
  programs.neovim.defaultEditor = true;
  users.users.herbst = {
    isNormalUser = true;
    home = "/home/herbst";
    description = "me";
    extraGroups = [ "networkmanager" "wheel" ];
    shell = pkgs.zsh;
  };
  environment.variables = {
    TERMINAL = "kitty";
    EDITOR = "nvim";
  };

#
# Packages
#
  services.flatpak.enable = true;
  nixpkgs.config.allowUnfree = true;
  environment.systemPackages = with pkgs; [
    # Utilities
    trash-cli
    git
    home-manager
    fastfetch
    pywal
    pulseaudio
    brightnessctl
    killall
    libnotify
    wineWowPackages.stable
    exfatprogs

    # Archiving Tools
    atool
    unrar
    unzip
    p7zip
    gnutar

    # Networking
    networkmanagerapplet
  
    # Build Tools
    dconf
    gnumake
    cmake
    meson
    cpio
    pkg-config

    # Wayland/X11 Utilities
    wl-clipboard
    wlr-randr
    xwayland-satellite
    #xclip
    j4-dmenu-desktop
    hyprpaper
    hyprnotify
    swaynotificationcenter
    waybar
    eww
    labwc

    # Theming & Appearance
    nwg-look
    tango-icon-theme
    simp1e-cursors
    feh

    # Applications
    libreoffice
    keepassxc
    discord
    btop
    yazi
    gucharmap
    firefox
    brave
    kitty
    dmenu-wayland
    tofi
    neovim
    wcalc
    gimp
    fsearch
    font-manager
    flameshot
    vscode
    bemenu
    lutris
    qbittorrent
  ];
 
  fonts.packages = with pkgs; [
    nerd-fonts.departure-mono
    nerd-fonts.jetbrains-mono
    nerd-fonts.gohufont
    terminus_font
    terminus_font_ttf
    siji
  ];

#
# Display Services
#

  services.displayManager.ly.enable = true;
  security.polkit.enable = true;
  programs.xwayland.enable = true;
  services.xserver = {
    enable = true;
  };
  services.picom = {
    enable = true;
    vSync = true;
    backend = "glx";
    settings = {
      glx-no-stencil = true;
      glx-copy-from-front = false;
    };
  };
    
  # Window Managers
  services.xserver.windowManager = {
    awesome = {
      enable = true;
    };
  };
  programs = {
    hyprland = { enable = true; };
    niri = { enable = true; };
  };

#
# Networking
#

  networking = {
    hostName = "nixos";
    networkmanager = {
      enable = true;
      dns = "systemd-resolved";
    };
  };
  services.resolved = {
    enable = true;
    dnssec = "true";
    fallbackDns = [ "1.1.1.1" "8.8.8.8" ];
    extraConfig = '' DNSOverTLS=opportunistic '';
  };
  networking.nameservers = [ "127.0.0.53" ];

#
# Boot Settings
#

  boot = {
    plymouth = {
      enable = true;
      theme = "nixos-bgrt";
      themePackages = with pkgs; [ nixos-bgrt-plymouth ];
    }; 
    # So-called "Silent Boot"
    consoleLogLevel = 0;
    initrd.verbose = false;
    kernelParams = [
      "quiet"
      "splash"
      #"boot.shell_on_fail"
      #"loglevel=3"
      #"rd.systemd.show_status=false"
      #"rd.udev.log_level=3"
      #"udev.log_priority=3"
    ];

    loader = {
      systemd-boot.enable = true;
      efi.canTouchEfiVariables = true;
    };
  };
  
#
# Hardware Stuff
#
  # Graphics Drivers
  hardware.graphics = {
      enable = true;
      enable32Bit = true;
  };

  # Enable printing with CUPS
  services.printing.enable = true;

  # Enable sound with pipewire.
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
  };

  # Maybe this is out of place IDK
  time.timeZone = "Asia/Jakarta";
  i18n.defaultLocale = "en_US.UTF-8"; 

  system.stateVersion = "24.11";

}
