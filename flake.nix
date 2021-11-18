{
  description = "now-collector";

  outputs = { self, nixpkgs }: let
    overlay = final: prev: {
      now-collector = final.callPackage (
        { rustPlatform }:

        rustPlatform.buildRustPackage {
          pname = "now-collector";
          version = self.shortRev or "dirty-${toString self.lastModifiedDate}";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
        }
      ) {};
    };
  in {
    inherit overlay;
    packages.x86_64-linux = import nixpkgs {
      system = "x86_64-linux";
      overlays = [ overlay ];
    };
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.now-collector;
  };
}
