# Maintainer: Oguz Kaganer Eren <oguzkaganeren@gmail.com>

pkgname=openrgb-ruler-git
pkgver=r17.f714d1a
pkgrel=1
pkgdesc="GUI for automated RGB lighting control rules via OpenRGB"
arch=('x86_64' 'aarch64')
url="https://github.com/oguzkaganeren/openrgb-ruler"
license=('GPL3')
depends=(
  'cairo'
  'dbus'
  'desktop-file-utils'
  'gdk-pixbuf2'
  'glib2'
  'gtk4'
  'hicolor-icon-theme'
  'pango'
  'openrgb'
)
makedepends=(
  'git'
  'rust'
  'cargo'
)
provides=('openrgb-ruler')
conflicts=('openrgb-ruler')
install=${pkgname%-git}.install
source=("git+${url}.git")
sha256sums=('SKIP')

pkgver() {
  cd openrgb-ruler
  (
    set -o pipefail
    git describe --long --abbrev=7 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
      printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
  )
}

build() {
  cd openrgb-ruler
  cargo build --release --locked
}

package() {
  cd openrgb-ruler

  install -Dm755 target/release/openrgb-ruler-gtk \
    "${pkgdir}/usr/bin/openrgb-ruler-gtk"

  install -Dm644 openrgb-ruler-gtk.desktop \
    "${pkgdir}/usr/share/applications/openrgb-ruler-gtk.desktop"

  install -Dm644 icons/32x32.png \
    "${pkgdir}/usr/share/icons/hicolor/32x32/apps/openrgb-ruler.png"

  install -Dm644 icons/64x64.png \
    "${pkgdir}/usr/share/icons/hicolor/64x64/apps/openrgb-ruler.png"

  install -Dm644 icons/128x128.png \
    "${pkgdir}/usr/share/icons/hicolor/128x128/apps/openrgb-ruler.png"

  install -Dm644 icons/icon.png \
    "${pkgdir}/usr/share/icons/hicolor/256x256/apps/openrgb-ruler.png"
}
