{
  description = "An insane ChatGPT client written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nix-systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, ... }@inputs:
    let
      eachSystem = nixpkgs.lib.genAttrs (import inputs.nix-systems);
      mkPkgs = system:
        import nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
    in {
      overlays.default =
        (final: prev: { rgpt = self.packages.${prev.system}.default; });
      packages = eachSystem (system:
        let pkgs = mkPkgs system;
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            name = "rgpt";
            version = "0.0.1";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
            buildInputs = with pkgs;
              if pkgs.stdenv.isDarwin then
                with darwin; [
                  apple_sdk.frameworks.SystemConfiguration
                  libiconv
                ]
              else
                [ ];
            meta = with pkgs.lib; {
              description = "An insane ChatGPT client written in Rust";
              homepage = "https://github.com/NewDawn0/rgpt";
              maintainers = with maintainers; [ "NewDawn0" ];
              license = licenses.mit;
            };
          };
        });
    };
}
