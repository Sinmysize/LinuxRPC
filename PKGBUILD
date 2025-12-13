# Maintainer: Sinmysize <sinmysize@gmail.com>
pkgname=linuxrpc
pkgver=1.1.0
pkgrel=1
pkgdesc="A Simple Discord RPC Client"
arch=('x86_64')
url="https://github.com/Sinmysize/LinuxRPC.git"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname::git+https://github.com/Sinmysize/LinuxRPC.git")
sha256sums=("SKIP")

prepare() {
    cd "$pkgname"

    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target $(rustc --print host-tuple)
}

build() {
    cd "$pkgname"

    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$pkgname"

    install -Dm755 -t "$pkgdir/usr/bin/" "target/release/$pkgname" 
    install -Dm644 "systemd/linuxrpc.service" \
        "$pkgdir/usr/lib/systemd/user/linuxrpc.service"
}