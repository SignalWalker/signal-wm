{
  description = "A dynamic, tiling Wayland compositor, designed for efficiency and low latency.";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    alejandra = {
      url = github:kamadorueda/alejandra;
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = github:nix-community/fenix;
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = github:nix-community/naersk;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ {
    self,
    nixpkgs,
    ...
  }:
    with builtins; let
      std = nixpkgs.lib;
      nixpkgsFor = std.genAttrs systems (system: import nixpkgs {
        localSystem = system;
        crossSystem = system;
        overlays = [ self.overlays.${system} ];
      });
    in {
      formatter = std.mapAttrs (system: pkgs: pkgs.default) inputs.alejandra.packages;
      overlays = std.genAttrs systems (system: final: prev: {
        signal-wm = {};
      });
      packages = std.genAttrs systems (system: let pkgs = nixpkgsFor.${system}; in {
        signal-wm = pkgs.signal-wm;
        default = self.packages.${system}.signal-wm;
      });
      apps = std.genAttrs systems (system: let pkgs = nixpkgsFor.${system}; in {
      });
    };
}
