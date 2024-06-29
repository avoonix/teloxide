{
  description = "teloxide";
  outputs = { self, nixpkgs }: {
    devShell.x86_64-linux =
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
      in pkgs.mkShell {
        buildInputs = [
          pkgs.openssl
          pkgs.pkg-config
        ];
        shellHook = ''
          echo 'to run tests: cargo test --features "full nightly"'
        '';
      };
  };
}
