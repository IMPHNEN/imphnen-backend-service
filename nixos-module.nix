{ config, lib, pkgs, ... }:
let
  cfg = config.services.imphnen-backend;
in
{
  options.services.imphnen-backend = {
    enable = lib.mkEnableOption "IMPHNEN backend service";

    port = lib.mkOption {
      type = lib.types.port;
      default = 8081;
      description = "Port the backend HTTP server listens on.";
    };

    environmentFile = lib.mkOption {
      type = lib.types.path;
      description = "Path to environment file with secrets (DATABASE_URL, JWT keys, etc).";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = false;
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.imphnen-backend = {
      description = "IMPHNEN Backend Service";
      wantedBy = [ "multi-user.target" ];
      after = [
        "network.target"
        "postgresql.service"
      ];
      serviceConfig = {
        ExecStart = "${pkgs.imphnen-backend}/bin/api";
        EnvironmentFile = cfg.environmentFile;
        Environment = [ "PORT=${toString cfg.port}" ];
        DynamicUser = true;
        Restart = "on-failure";
        RestartSec = "5s";
        StandardOutput = "journal";
        StandardError = "journal";
      };
    };

    networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ cfg.port ];
  };
}
