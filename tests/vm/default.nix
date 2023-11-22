# add-hoc config for a test VM. 
{ inputs, modulesPath, ... }:
{
  imports = [
    "${modulesPath}/virtualisation/qemu-vm.nix"
    ./lizard.nix
  ];

  fileSystems = {
    "/" = {
      device = "/dev/disk/by-label/nixos";
      fsType = "ext4";
    };
  };

  boot = {
    kernelParams = [ "console=ttyS0" "boot.shell_on_fail" ];
    loader.timeout = 5;
  };

  virtualisation = {
    diskSize = 512;
    memorySize = 512;
    graphics = false;
  };

  services.getty = {
    autologinUser = "root";
  };
  users.motd = ''
  Trekkie-McTest: enterprise-grade, free-range, grass-fed testing vm
  Now with 100% less graphics!

  Services exposed to the host:
  lizard: 8060
  SSH: 2222
  redis: 8061

  root password is "lol"

  have fun!
  '';


  networking.firewall.enable = false;

  users.mutableUsers = false;
  users.users.root.password = "lol";
  services.openssh = {
    enable = true;
    permitRootLogin = "yes";
  };

  system.stateVersion = "22.11";
}
