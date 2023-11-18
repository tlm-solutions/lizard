{ buildPackage, lib, pkg-config, cmake, protobuf, postgresql, zlib, openssl}:

buildPackage {
  pname = "lizard";
  version = "0.1.0";

  src = ./.;

  #cargoSha256 = lib.fakeSha256;
  buildInputs = [ cmake protobuf openssl pkg-config postgresql ];

  meta = {
    description = "Service which serves the current state of the network";
    homepage = "https://github.com/tlm-solutions/lizard";
  };
}
