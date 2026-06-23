# Maintainer: hisoka <root@hisoka.lol>
pkgname=rsstoat
pkgver=1.3.0
pkgrel=1
pkgdesc="rsStoat - Tauri v2 desktop client for Stoat chat"
arch=('x86_64')
url="https://github.com/hisoka-root/rsstoat"
license=('MPL')
depends=(
  'webkit2gtk-4.1'
  'libsoup3'
  'gtk3'
  'glib2'
  'cairo'
  'gdk-pixbuf2'
  'pango'
  'atk'
  'librsvg'
  'libappindicator-gtk3'
)
makedepends=(
  'rust'
  'cargo'
  'nodejs'
  'pnpm'
)
source=("$pkgname-$pkgver.tar.gz::https://github.com/hisoka-root/rsstoat/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$srcdir/rsstoat-$pkgver"
  pnpm install
  cargo tauri build
}

package() {
  cd "$srcdir/rsstoat-$pkgver"
  install -Dm755 "src-tauri/target/release/rsstoat" "$pkgdir/usr/bin/rsstoat"
  install -Dm644 "chat.rsstoat.RsStoat.desktop" "$pkgdir/usr/share/applications/chat.rsstoat.RsStoat.desktop"
  install -Dm644 "chat.rsstoat.RsStoat.metainfo.xml" "$pkgdir/usr/share/metainfo/chat.rsstoat.RsStoat.metainfo.xml"
  install -Dm644 "src-tauri/icons/128x128.png" "$pkgdir/usr/share/icons/hicolor/128x128/apps/rsstoat.png"
  install -Dm644 "src-tauri/icons/32x32.png" "$pkgdir/usr/share/icons/hicolor/32x32/apps/rsstoat.png"
}
