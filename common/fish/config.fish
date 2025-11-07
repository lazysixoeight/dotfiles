functions --erase fish_command_not_found
set fish_greeting

alias initdev="nix flake init --template 'github:the-nix-way/dev-templates#empty'"

# Environment Variables
export LC_COLLATE=C

if status is-interactive
end
