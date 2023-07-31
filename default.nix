{ lib
, stdenv
, docker
, fetchurl
, openssl 
, rustfmt
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "anisette-server";
  version = "0.1";

  src = fetchurl {
    url = "https://github.com/max-amb/anisette-server/archive/refs/tags/v${version}.tar.gz";
    hash = "sha256-u2z/z1K8Ok9m19mPW/bzhZI2IErv0JHUzdyrKhJWYN0=";
  };

  cargoSha256 = "sha256-IfcJBDoFoIiWa3hSUm7WyeNGgaNFrnYFOlLoLM5bHaA=";

  OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
  OPENSSL_LIB_DIR = "${lib.getLib openssl}/lib";

  nativeBuildInputs = [ rustfmt ];

  buildInputs = [
    docker
  ]

  meta = with lib; {
    homepage = "https://github.com/max-amb/anisette-server";
    description = "alt-server-anisette-server is a simple program to streamline the management of the docker crate nyamisty/alt_anisette_server";
    license = licenses.gpl3Only;
    platforms = platforms.linux;
    maintainers = with maintainers; [ max-amb ];
  };
}
