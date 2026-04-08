# Maintainer: leessmin <1442772970@qq.com>

pkgname=random-food
pkgver=0.2.0
pkgrel=1
pkgdesc="一个随机食物生成器"
arch=('x86_64')
url="https://github.com/leessmin/random-food"
license=('GPLv3')
options=('!strip')
source=("https://github.com/leessmin/random-food/releases/download/0.0.1/random-food")
sha256sums=('c54236b40d1d3644015ef9e8be8a9093ad157e797c560a8d61ee14e036814439')

package() {
        install -Dm755 "random-food" "$pkgdir/usr/bin/random-food"
}
