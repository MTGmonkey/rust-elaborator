{
  openssl,
  pkg-config,
  naersk,
  ...
}:
naersk.buildPackage {
  name = "rust_elaborator";
  src = ./.;
  buildInputs = [openssl];
  nativeBuildInputs = [pkg-config];
  meta = {
    mainProgram = "rust_elaborator";
    homepage = "https://mtgmonkey.net";
  };
}
