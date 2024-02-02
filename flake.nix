{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.rustPlatform.buildRustPackage {
        pname = "gh-grader-preview";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
            lockFile = ./Cargo.lock;
        };
        doCheck = false;
        meta = with nixpkgs.legacyPackages.x86_64-linux.lib; {
            description = "Simple program for previewing how GitHub Classroom runs your project ";
            homepage = "https://github.com/Bwc9876/gh-grader-preview";
            mainProgram = "gh-grader-preview";
            license = licenses.mit;
        };
    };
  };
}
