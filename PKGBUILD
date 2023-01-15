# Maintainer: Jasper <j@sperp.dev> -> https://github.com/jaspwr

pkgname=tx
pkgver=0.0.1
pkgrel=1
pkgdesc="A converter for Nick Land's Tic Xenotation."
makedepends=('rust' 'cargo')
license=('GPL3')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url="https://github.com/jaspwr/Tic-Xenotation-Translator"

source=("$pkgname::git+https://github.com/jaspwr/Tic-Xenotation-Translator#branch=main")
sha256sums=('SKIP')

prepare() {
    cd "Tic-Xenotation-Translator"
    cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "Tic-Xenotation-Translator"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --release
}

package() {
    cd "Tic-Xenotation-Translator"
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/tx"
}
