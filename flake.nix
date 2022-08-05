{
  description = "A dynamic Wayland compositor, designed for efficiency and low latency. Built with Rust and WGPU.";
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
      systems = ["x86_64-linux"];
      nixpkgsFor = std.genAttrs systems (system:
        import nixpkgs {
          localSystem = system;
          crossSystem = system;
          overlays = [self.overlays.${system}];
        });
    in {
      formatter = std.mapAttrs (system: pkgs: pkgs.default) inputs.alejandra.packages;
      overlays = std.genAttrs systems (system: final: prev: let
        naersk = inputs.naersk.lib.${system}.override {
          inherit (inputs.fenix.packages.${system}.minimal) cargo rustc;
        };
      in {
        signal-wm = naersk.buildPackage {
          src = ./.;
          nativeBuildInputs = with final; [ pkg-config dbus udev ];
        };
      });
      packages = std.genAttrs systems (system: let
        pkgs = nixpkgsFor.${system};
      in {
        signal-wm = pkgs.signal-wm;
        default = self.packages.${system}.signal-wm;
      });
      # apps = std.genAttrs systems (system: let
      #   pkgs = nixpkgsFor.${system};
      # in {
      # });
    };
}
