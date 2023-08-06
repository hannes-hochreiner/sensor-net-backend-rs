{
  description = "SensorNet backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};
        sensor-net-backend = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;

          nativeBuildInputs = with pkgs; [
            # Add additional build inputs here
            gcc
            fontconfig
            freetype
            pkgconf
            pkg-config-unwrapped
            expat
          ];

          buildInputs = with pkgs; [
            freetype
          ];

          PKG_CONFIG_PATH = "${pkgs.expat.dev}/lib/pkgconfig:${pkgs.fontconfig.dev}/lib/pkgconfig:${pkgs.freetype.dev}/lib/pkgconfig";
        };
      in
      {
        checks = {
          inherit sensor-net-backend;
        };

        packages.default = sensor-net-backend;

        apps.default = flake-utils.lib.mkApp {
          drv = sensor-net-backend;
        };

        nixosModules.default = { config, lib, pkgs, ... }:
          with lib;
          let cfg = config.hochreiner.services.sensor-net-backend;
          in {
            options.hochreiner.services.sensor-net-backend = {
              enable = mkEnableOption "Enables the sensor-net-backend service";
              env_path = mkOption {
                type = lib.types.path;
                description = "Sets the path of the event-extractor environment file";
              };
              user = mkOption {
                type = lib.types.str;
                description = "Sets the user for the service";
              };
              group = mkOption {
                type = lib.types.str;
                description = "Sets the group for the service";
              };
            };

            config = mkIf cfg.enable {
              systemd.services."hochreiner.sensor-net-backend" = {
                description = "SensorNet backend service";
                wantedBy = [ "multi-user.target" ];

                serviceConfig = let pkg = self.packages.${system}.default;
                in {
                  Restart = "on-failure";
                  Type = "simple";
                  ExecStart = "${pkg}/bin/sensor-net-backend";
                  EnvironmentFile = cfg.env_path;
                  User = cfg.user;
                  Group = cfg.group;
                };
              };
            };
          };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            gcc
            fontconfig
            freetype
            pkgconf
          ];
        };
      }
    );
  
  nixConfig = {
    substituters = [
      "https://cache.nixos.org"
      "https://hannes-hochreiner.cachix.org"
    ];
    trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "hannes-hochreiner.cachix.org-1:+ljzSuDIM6I+FbA0mdBTSGHcKOcEZSECEtYIEcDA4Hg="
    ];
  };
}