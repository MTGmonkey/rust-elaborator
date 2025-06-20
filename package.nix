{rustPlatform, ...}:
rustPlatform.buildRustPackage {
  name = "rust_elaborator";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  nativeBuildInputs = [];
  buildInputs = [];
  configurePhase = '''';
  buildPhase = '''';
  installPhase = '''';
  meta = {
    mainProgram = "rust_elaborator";
    homepage = "https://mtgmonkey.net";
  };
}
