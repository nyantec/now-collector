{
  description = "now-collector";

  outputs = { self, nixpkgs }: let
    version = self.shortRev or (toString self.lastModifiedDate);
    overlay = final: prev: {
      now-collector = final.callPackage (
        { rustPlatform }: rustPlatform.buildRustPackage {
          pname = "now-collector";
          inherit version;
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
        }
      ) {};

      now-collector-pkg = final.callPackage (
        { now-collector, zstd }: pkgs.runCommand "now-collector-pkg" {
          nativeBuildInputs = [ zstd ];
        } ''
          mkdir -p usr/bin $out
          cp ${now-collector}/bin/now-collector usr/bin
          tar --zstd -cf $out/now-collector-x86_64-${now-collector.version}.pkg usr
        ''
      ) {};
    };
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      crossSystem = {
        isStatic = true;
        config = "x86_64-unknown-linux-musl";
      };
      overlays = [ overlay ];
    };
  in {
    inherit overlay;
    packages.x86_64-linux = {
      inherit (pkgs) now-collector now-collector-pkg;
    };
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.now-collector-pkg;
  };
}
