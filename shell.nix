with import <nixpkgs> {};

mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    pkg-config
    gcc
    atkmm
    glib
    cairomm
    gdk-pixbuf
    gsettings-desktop-schemas
    gtk3
    vscode
    hicolor-icon-theme
 ];

  shellHook =
    ''
      export GIT_SSL_CAINFO=/etc/ssl/certs/ca-certificates.crt
      export SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
      export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH
    '';
  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${ with pkgs; lib.makeLibraryPath [
    wayland
    libxkbcommon
    gcc
    libGL
    gsettings-desktop-schemas
    gtk3
    hicolor-icon-theme
    atkmm
    glib
    cairomm
    gdk-pixbuf
  ] }";
}
