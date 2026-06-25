# Maintainer: Sinmysize <sinmysize@gmail.com>
pkgname=linuxrpc
pkgver=2.1.0
pkgrel=1
pkgdesc="A Simple Discord RPC Client"
arch=(any)
url="https://github.com/Sinmysize/LinuxRPC.git"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname::git+https://github.com/Sinmysize/LinuxRPC.git")
sha256sums=("SKIP")

prepare() {
    cd "$pkgname"

    cargo fetch --locked --target $(rustc --print host-tuple)
}

build() {
    cd "$pkgname"
    
    cargo build --frozen --release --all-features
}

package() {
    cd "$pkgname"

    install -Dm755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}