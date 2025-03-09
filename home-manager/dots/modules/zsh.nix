{config, ...}:
{
  programs.zsh = {
    enable = true;
    autosuggestion = {
      enable = true;
      highlight = "fg=#999999";
      strategy = ["history" "completion"];
    };
    shellAliases = {
      nix-shell = "nix-shell /etc/nixos";
    };
    oh-my-zsh = {
      enable = true;
      theme = "fishy";
    };
  };
}
