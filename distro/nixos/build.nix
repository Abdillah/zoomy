args@{ stdenv, lib, fetchurl, fetchFromGitHub, rustPlatform,
  pkgconfig, cargo, rustc, libdrm,
  # Plugins
  ... }:

let
  self = rustPlatform.buildRustPackage rec {
    name = "zoomy-${version}";
    version = "0.0.1";

    src = fetchurl {
      url = "file:///home/fazbdillah/opengles/Zoomy/zoomy.tar.gz";
      sha256 = "14iig925j5xqn6hjr90m4dxrz7pfrhypxm8m726yh8xqa4ypcbmd";
    };

    nativeBuildInputs = [ pkgconfig rustc cargo ];
    buildInputs = [ libdrm ];

    depsSha256 = "08riayb1lbqcz2nm2pf5lkb6chi971f4prqzg64hf18f8m4rb889";

    doCheck = false;
  };
in self

