export def update-flake-lock [] {
  ^podman "run" "--rm" "-it" "-v" $"($env.PWD):/workspace:z" "nixos/nix" "bash" "-c" "nix flake update --extra-experimental-features nix-command --extra-experimental-features flakes --accept-flake-config /workspace"
}
export def start-nix [] {
  # podman run --rm -it -v $"($env.PWD):/workspace:z" nixos/nix bash -c "nix build --extra-experimental-features nix-command --extra-experimental-features flakes --recreate-lock-file --accept-flake-config /workspace"
  podman run --rm -it -v $"($env.PWD):/workspace:z" nixos/nix bash
}