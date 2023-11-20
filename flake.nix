{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    utils = {
      url = "github:numtide/flake-utils";
    };

    fenix = {
      url = "github:nix-community/fenix";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, naersk, fenix }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          toolchain = with fenix.packages.${system}; combine [
            latest.cargo
            latest.rustc
          ];

          package = pkgs.callPackage ./derivation.nix {
            buildPackage = (naersk.lib.${system}.override {
              cargo = toolchain;
              rustc = toolchain;
            }).buildPackage;
          };
        in
        rec {
          checks = packages;
          packages = {
            lizard = package;
            default = package;
            docs = (pkgs.nixosOptionsDoc {
              options = (nixpkgs.lib.nixosSystem {
                inherit system;
                modules = [ self.nixosModules.default ];
              }).options.TLMS;
            }).optionsCommonMark;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = (with packages.lizard; nativeBuildInputs ++ buildInputs);
          };

          apps = {
            lizard = utils.lib.mkApp { drv = packages.lizard; };
            default = apps.lizard;
          };
        }) // {
      nixosModules = rec {
        default = funnel;
        funnel = import ./nixos-module;
      };
      overlays.default = final: prev: {
        inherit (self.packages.${prev.system})
          lizard;
      };
    };
}
