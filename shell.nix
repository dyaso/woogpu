with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
        cargo rustc

  	    # libxkbcommon

        # for druid
       #  cairo
       #  pango
       #  atk
       #  gdk-pixbuf
       #  gtk3-x11

       #  #needed for `shello` example
       #  glib
  	    x11
  	    pkgconfig
  vulkan-loader

#luajit
];
  buildInputs = [
  #x11
  	    xorg.libXcursor
  	    xorg.libXrandr
  	    xorg.libXi

cmake # needed to make the shader compiler thing? shaderc-sys(build)


];

  RUST_BACKTRACE = 1;

  #find this using: 
  #  nix eval --raw nixpkgs.vulkan-loader;
#  LD_LIBRARY_PATH = /nix/store/602kxzwr1imj5zvncd2zhnjqcs2f12is-vulkan-loader-1.2.131.2;


  }