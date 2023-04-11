{ craneLib, src, lib, cmake, pkg-config, protobuf, grpc, openssl, postgresql}:

craneLib.buildPackage {
  pname = "lizard";
  version = "0.1.0";

  src = ./.;

  buildInputs = [ cmake protobuf grpc openssl pkg-config postgresql ];

  meta = {
    description = "Service which serves the current state of the network";
    homepage = "https://github.com/tlm-solutions/lizard";
  };
}
