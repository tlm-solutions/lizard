{ pkgs, config, lib, ... }:
let
  cfg = config.TLMS.lizard;
in
{
  options.TLMS.lizard = with lib; {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "enabling the service";
    };
    redis = {
      host = mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = "grpc host";
      };
      port = mkOption {
        type = types.int;
        default = 50051;
        description = "grpc port";
      };
    };
    http = {
      host = mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = "http host";
      };
      port = mkOption {
        type = types.port;
        default = 9001;
        description = "port of the lizard";
      };
    };
    user = mkOption {
      type = types.str;
      default = "lizard";
      description = "as which user lizard should run";
    };
    group = mkOption {
      type = types.str;
      default = "lizard";
      description = "as which group lizard should run";
    };
    logLevel = mkOption {
      type = types.str;
      default = "info";
      description = "log level";
    };
    workerCount = mkOption {
      type = types.int;
      default = 4;
      description = "amount of worker threads used";
    };
  };
  config = lib.mkIf cfg.enable {
    systemd = {
      services = {
        "lizard" = {
          enable = true;
          wantedBy = [ "multi-user.target" "redis.service" ];

          script = "exec ${pkgs.lizard}/bin/lizard --host ${cfg.http.host} --port ${toString cfg.http.port}&";

          environment = {
            "RUST_BACKTRACE" = "1";
            "RUST_LOG" = "${cfg.logLevel}";
            "REDIS_HOST" = "${cfg.redis.host}";
            "REDIS_PORT" = "${toString cfg.redis.port}";
            "HTTP_PORT" = "${toString cfg.http.port}";
            "WORKER_COUNT" = "${toString cfg.workerCount}";
          };

          serviceConfig = {
            Type = "forking";
            User = "${cfg.user}";
            Restart = "always";
          };
        };
      };
    };

    # user accounts for systemd units
    users.users."${cfg.user}" = {
      name = "${cfg.user}";
      description = "public dvb api service";
      group = "${cfg.group}";
      isSystemUser = true;
      extraGroups = [ ];
    };
    users.groups."${cfg.group}" = {};
  };
}
