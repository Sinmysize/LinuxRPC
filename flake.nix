{
  description = "A simple and customizable Discord RPC client for Linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "linuxrpc";
          version = "2.4.0";
          src = pkgs.fetchFromGitHub {
            owner = "Sinmysize";
            repo = "LinuxRPC";
            rev = "master";
            hash = "sha256-gRYKoWrBgR1+/qz/oBCX9wknF7g5o7fla3aBF5E05N8=";
          };
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = [ ];
          postInstall = ''
            wrapProgram $out/bin/linuxrpc \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  pkgs.playerctl
                  pkgs.procps
                ]
              }
          '';
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          packages = [
            pkgs.cargo
            pkgs.rustc
          ];
        };
      }
    );
}
