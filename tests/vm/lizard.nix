{ config, inputs, ... }:
{
  TLMS.lizard = {
    # Unlike the production, we do not reverse-proxy the lizard, we just expose
    # port directly to the host vm.
    enable = true;
    http = {
      host = "0.0.0.0";
      port = 8060;
    };
    redis = {
      port = 6379;
      host = "localhost";
    };
    logLevel = "info";
  };
  systemd.services."lizard" = {
    after = [ "redis-lizard.service" ];
    wants = [ "redis-lizard.service" ];
  };

  services = {
    redis.servers."lizard" = {
      enable = true;
      bind = config.TLMS.lizard.redis.host;
      port = config.TLMS.lizard.redis.port;
    };
  };
}
