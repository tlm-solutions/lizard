{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, crane }:
    utils.lib.eachDefaultSystem
    (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.lib.${system};
      package = pkgs.callPackage ./derivation.nix { craneLib = craneLib; };
    in
    rec {
    checks = packages;
    packages = {
      lizard = package;
      default = package;
    };
    nixosModules = rec {
      default = funnel;
      funnel = import ./nixos-module;
    };
    devShells.default = pkgs.mkShell {
      nativeBuildInputs = (with packages.lizard; nativeBuildInputs ++ buildInputs);
    };
    apps = {
      lizard = utils.lib.mkApp { drv = packages.lizard; };
      default = apps.lizard;
    };
  }) // {
    overlays.default = final: prev: {
      inherit (self.packages.${prev.system})
      lizard;
    };
  };
}
