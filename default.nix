{ pkgs ? import (fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/77ef7a29d276c6d8303aece3444d61118ef71ac2.tar.gz";
    sha256 = "0pm4l48jq8plzrrrisimahxqlcpx7qqq9c99hylmf7p3zlc3phsy";
  }) {},
}:

let
  tauriLibs = with pkgs; [
    webkitgtk_4_1
    libsoup_3
    gtk3
    javascriptcoregtk_4_1
    cairo
    pango
    atk
    gdk-pixbuf
    glib
    librsvg
    libappindicator-gtk3
    libayatana-appindicator
  ];
in

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    # Tools
    pkgs.mise

    # Rust toolchain
    rustup

    # build utils
    pkgs.zip
    pkgs.dpkg
    pkgs.fakeroot
    pkgs.patchelf

    # Tauri system libs
  ] ++ tauriLibs;

  shellHook = ''
    export RUSTUP_HOME="$HOME/.rustup"
    export CARGO_HOME="$HOME/.cargo"
    export PATH="$CARGO_HOME/bin:$PATH"
    export WEBKIT_DISABLE_COMPOSITING_MODE=1
    export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:$XDG_DATA_DIRS"
    eval "$(mise activate bash)"
  '';
}
