{
  openssl,
  pkg-config,
  stdenv,
  naersk,
  ...
}:
naersk.buildPackage {
  name = "rust_elaborator";
  src = ./.;
  buildInputs = [openssl];
  nativeBuildInputs = [pkg-config];
  #  configurePhase = '''';
  #  buildPhase = '''';
  #  installPhase = ''
  #    install -Dm775 ./target/release/rust_elaborator $out/bin/rust_elaborator
  #  '';
  meta = {
    mainProgram = "rust_elaborator";
    homepage = "https://mtgmonkey.net";
  };
}
