# Maintainer: leessmin <1442772970@qq.com>

pkgname=random-food
pkgver=0.2.0
pkgrel=1
pkgdesc="一个随机食物生成器"
arch=('x86_64')
url="https://github.com/leessmin/random-food"
license=('GPLv3')
options=('!strip')
source=("https://github.com/leessmin/random-food/releases/download/0.2.0/random-food")
sha256sums=('c553aadfb76157a52a374f22c2fec1fe34d6399ca58eefff47b26e0e715066eb')

package() {
        install -Dm755 "random-food" "$pkgdir/usr/bin/random-food"
}
