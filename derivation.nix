{ craneLib, src, lib, cmake, pkg-config, protobuf, grpc, openssl, postgresql}:

craneLib.buildPackage {
  pname = "lizard";
  version = "0.1.0";

  src = ./.;

  buildInputs = [ cmake protobuf grpc openssl pkg-config postgresql ];

  meta = {
    description = "Tool to correlate the r09 telegrams to transmission locations";
    homepage = "https://github.com/dump-dvb/lofi";
  };
}
