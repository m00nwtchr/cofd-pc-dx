{
  description = "Chronicles of Darkness Player Companion Nix Flake";

  inputs = {
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    android,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      inherit (pkgs) lib;

      # rustToolchainFor = p: (p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      rustToolchainFor = p:
        p.rust-bin.selectLatestNightlyWith (
          t: t.default
        );
      rustDevToolchainFor = p:
        (rustToolchainFor p).override {
          extensions = ["rust-src" "rust-analyzer"];
        };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;
      craneDev = craneLib.overrideToolchain rustDevToolchainFor;

      src = lib.cleanSourceWith {
        src = self; # The original, unfiltered source
        filter = path: type:
          !(lib.hasSuffix "tailwind.css" path) # Generated CSS
          && ((lib.hasSuffix "input.css" path) # Tailwind CSS Input
            || (lib.hasInfix "/assets/" path) # Assets
            || (craneLib.filterCargoSources path type)); # .rs
      };

      # Common arguments can be set here to avoid repeating them later
      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs = with pkgs;
          (lib.optionals stdenv.isLinux
            [
              webkitgtk_4_1
              xdotool
              wayland
            ])
          ++ lib.optionals stdenv.isDarwin (
            with darwin.apple_sdk.frameworks; [
              IOKit
              Carbon
              WebKit
              Security
              Cocoa
            ]
          );

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      cofd-pc = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;

          nativeBuildInputs = with pkgs;
            commonArgs.nativeBuildInputs
            ++ [
              tailwindcss_4
              dioxus-cli
            ];

          preBuild = ''
            tailwindcss -i input.css -o assets/tailwind.css
          '';

          cargoBuildCommand = "dx build --release --platform desktop --";
          installPhaseCommand = ''
            install -Dm755 target/dx/cofd-pc/release/linux/app/cofd-pc $out/bin/cofd-pc
          '';

          postInstall = ''
            cp -a target/dx/cofd-pc/release/linux/app/assets $out/bin/
          '';
        });
    in {
      checks = {
        inherit cofd-pc;

        # Run clippy
        cofd-pc-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        # Check formatting
        cofd-pc-fmt = craneLib.cargoFmt {
          inherit src;
        };

        cofd-pc-deny = craneLib.cargoDeny {
          inherit src;
        };
      };

      packages.default = cofd-pc;
      apps.default = flake-utils.lib.mkApp {
        drv = cofd-pc;
      };

      devShells.default = craneDev.devShell {
        checks = self.checks.${system};

        packages = [
          pkgs.wasm-bindgen-cli_0_2_100
        ];
      };
    });
}
