//! メッシュLOD段1つぶんの中身の量と、そこから決まるバッファの区間の長さ。
//! 受け取るのは頂点数とインデックス数、返すのは区間ごとのバイト長と段1つぶんの合計である。
//! 長さの決め方をここが持つのは、粗い原型と細分化した診断原型で違うのがこの2つの数だけであり、
//! 区間の並び順も属性の組(位置VEC3・法線VEC3・接線VEC4・UV VEC2・インデックスu16)も同じだからである。

/// 位置と法線のf32三成分。
const 三成分のバイト長: usize = 12;
/// 接線のf32四成分。
const 四成分のバイト長: usize = 16;
/// テクスチャ座標のf32二成分。
const 二成分のバイト長: usize = 8;
/// インデックス1つぶんのu16。
const インデックスのバイト長: usize = 2;

#[derive(Clone, Copy)]
pub(super) struct 段の中身の量 {
    pub(super) 頂点数: usize,
    pub(super) インデックス数: usize,
}

impl 段の中身の量 {
    pub(super) const fn 位置区間長(self) -> usize {
        self.頂点数 * 三成分のバイト長
    }

    pub(super) const fn 法線区間長(self) -> usize {
        self.頂点数 * 三成分のバイト長
    }

    pub(super) const fn 接線区間長(self) -> usize {
        self.頂点数 * 四成分のバイト長
    }

    pub(super) const fn テクスチャ座標区間長(self) -> usize {
        self.頂点数 * 二成分のバイト長
    }

    pub(super) const fn インデックス区間長(self) -> usize {
        self.インデックス数 * インデックスのバイト長
    }

    /// 段1つぶんのバイト長。次の段の区間はこの長さの倍数から始まる。
    pub(super) const fn バイト長(self) -> usize {
        self.位置区間長() + self.法線区間長() + self.接線区間長() + self.テクスチャ座標区間長() + self.インデックス区間長()
    }
}
