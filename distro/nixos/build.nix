args@{ stdenv, lib, fetchurl, fetchFromGitHub, rustPlatform,
  pkgconfig, cargo, rustc, libdrm,
  # Plugins
  ... }:

let
  self = rustPlatform.buildRustPackage rec {
    name = "zoomy-${version}";
    version = "0.0.1";

    src = fetchFromGitHub {
      owner = "Abdillah";
      repo = "zoomy";
      rev = "505c2bb77824b492464f8649bcf8478d785b0370";
      sha256 = "1gkcmfvdivpgpzr261i3473rwrb89jjg0p49glgrq923bbddnlb5";
    };

    nativeBuildInputs = [ pkgconfig rustc cargo ];
    buildInputs = [ libdrm ];

    depsSha256 = "08riayb1lbqcz2nm2pf5lkb6chi971f4prqzg64hf18f8m4rb889";

    doCheck = false;
  };
in self

