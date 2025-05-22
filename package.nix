{ lib, rustPlatform, pkg-config, cmake, protobuf, openssl, libpq, ... }:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = manifest.name;
  inherit (manifest) version;

  src = lib.cleanSource ./.;

  cargoHash = "sha256-3EdS3A19ZMU1tzgrx6A+5c/1fpkHY1hwU9j5De6sTm8=";

  cargoBuildFlags = "-p ${finalAttrs.pname}";
  cargoTestFlags = "-p ${finalAttrs.pname}";

  nativeBuildInputs = [ pkg-config cmake protobuf ];

  buildInputs = [ openssl libpq ];

  meta = {
    mainProgram = "chemo";
    description = "Service which serves the current state of the network";
    homepage = "https://github.com/tlm-solutions/lizard";
  };
})

