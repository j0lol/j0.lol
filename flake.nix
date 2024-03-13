# in flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # 👇 we have a new input!
    flake-utils.url = "github:numtide/flake-utils";
  };
  #      it's destructured here 👇
  outputs = { self, nixpkgs, flake-utils }:
    # calling a function from `flake-utils` that takes a lambda
    # that takes the system we're targetting
    flake-utils.lib.eachDefaultSystem (system:
      let
        # no need to define `system` anymore
        name = "simple";
        src = ./.;
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # `eachDefaultSystem` transforms the input, our output set
        # now simply has `packages.default` which gets turned into
        # `packages.${system}.default` (for each system)
        packages.default = derivation {
          inherit system name src;
          builder = with pkgs; "${bash}/bin/bash";
          args = [ "-c" "echo foo > $out" ];
        };
      }
    );
}
