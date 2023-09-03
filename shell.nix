let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      ref = "master";
    });
  pinned = builtins.fetchGit {
    url = "https://github.com/nixos/nixpkgs/";
    ref = "master";
  };
  nixpkgs = import pinned { overlays = [ mozillaOverlay ]; };
  rust = with nixpkgs; (rustChannelOf { channel = "nightly"; date = "2023-09-03";}).rust;
in
with nixpkgs; pkgs.mkShell {
  buildInputs = [
    clang
    pkg-config
    openssl
    rust
  ] ++ (lib.optionals stdenv.isDarwin [
    darwin.libiconv
    darwin.apple_sdk.frameworks.Security
  ]);

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library/";
}
