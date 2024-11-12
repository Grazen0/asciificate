{
  description = "Blazingly fast image-to-ASCII converter";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = nixpkgs.lib.genAttrs [
      "aarch64-linux"
      "i686-linux"
      "x86_64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ];

    pkgsFor = nixpkgs.legacyPackages;
  in {
    packages = forAllSystems (system: rec {
      asciify = pkgsFor.${system}.callPackage ./default.nix {};
      default = asciify;
    });

    devShells = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./shell.nix {};
    });
  };
}
