{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          package = pkgs.callPackage ./package.nix { };

          test-vm-pkg = self.nixosConfigurations.lizard-mctest.config.system.build.vm;
        in
        rec {
          checks = packages;
          packages = {
            lizard = package;
            test-vm = test-vm-pkg;
            test-vm-wrapper = pkgs.writeScript "trekkie-test-vm-wrapper"
              ''
                set -e

                echo Trekkie-McTest: enterprise-grade, free-range, grass fed testing vm
                echo
                echo "ALL RELEVANT SERVICES WILL BE EXPOSED TO THE HOST:"
                echo -e "Service\t\tPort"
                echo -e "SSH:\t\t2222\troot:lol"
                echo -e "lizard:\t8060"
                echo -e "redis:\t\t8061"
                echo

                set -x
                export QEMU_NET_OPTS="hostfwd=tcp::2222-:22,hostfwd=tcp::8060-:8060,hostfwd=tcp::8061-:6379"

                echo "running the vm now..."
                ${self.packages.${system}.test-vm}/bin/run-nixos-vm
              '';
            default = package;
            docs = (pkgs.nixosOptionsDoc {
              options = (nixpkgs.lib.nixosSystem {
                inherit system;
                modules = [ self.nixosModules.default ];
              }).options.TLMS;
            }).optionsCommonMark;
          };

          # to get yourself a virtualized testing playground:
          # nix run .\#mctest
          apps = {
            mctest = {
              type = "app";
              program = "${self.packages.${system}.test-vm-wrapper}";
            };
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
        default = lizard;
        lizard = import ./nixos-module;
      };
      overlays.default = final: prev: {
        inherit (self.packages.${prev.system})
          lizard;
      };

      # qemu vm for testing
      nixosConfigurations.lizard-mctest = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; };
        modules = [
          self.nixosModules.default
          ./tests/vm

          {
            nixpkgs.overlays = [
              self.overlays.default
            ];
          }
        ];
      };
    };
}
