# Maintainer: Sinmysize <sinmysize@gmail.com>
pkgname=linuxrpc
pkgver=1.0.0
pkgrel=1
pkgdesc="A Simple Discord RPC Client"
arch=('x86_64')
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("linuxrpc.service")
sha256sums=("SKIP")

build() {
    cd "$startdir"
    cargo build --release --locked
}

package() {
    cd "$startdir"
    
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    
    install -Dm644 "$srcdir/linuxrpc.service" \
        "$pkgdir/usr/lib/systemd/system/linuxrpc.service"
}