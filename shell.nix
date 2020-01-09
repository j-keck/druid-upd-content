{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {

  buildInputs = (with pkgs; [
    pkgconfig openssl
  ]) ++
  # droid
  (with pkgs; [
    gnome3.gtk glib cairo pango atk gdk_pixbuf
  ]);
}
