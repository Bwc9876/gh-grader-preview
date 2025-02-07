{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.rustPlatform.buildRustPackage rec {
        pname = "gh-grader-preview";
        version = "0.2.0";
        src = ./.;
        useFetchCargoVendor = true;
        cargoLock = {
            lockFile = ./Cargo.lock;
        };
        nativeBuildInputs = [
            nixpkgs.legacyPackages.x86_64-linux.installShellFiles
        ];
        postInstall = ''
            $out/bin/${pname} --man-page > ${pname}.0
            installManPage ${pname}.0
            $out/bin/${pname} --completions=bash > ${pname}.bash
            $out/bin/${pname} --completions=zsh > ${pname}.zsh
            $out/bin/${pname} --completions=fish > ${pname}.fish
            installShellCompletion gh-grader-preview.{bash,zsh,fish}
        '';
        doCheck = false;
        meta = with nixpkgs.legacyPackages.x86_64-linux.lib; {
            description = "Simple program for previewing how GitHub Classroom runs your project ";
            homepage = "https://github.com/Bwc9876/gh-grader-preview";
            license = licenses.mit;
        };
    };
  };
}
