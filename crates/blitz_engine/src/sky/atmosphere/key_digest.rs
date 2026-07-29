//! 大気の値を1つの64ビット整数へ畳み込む工程。受け取るのは単位付きの値、返すのは畳み込み済みの整数である。
//!
//! 畳み込みにFNV-1aを自前で書くのは、標準ライブラリの`DefaultHasher`が版をまたいだ出力の同一性を約束していないためである。
//! 大気静的キーはLUTを作り直すかどうかの判定に使い、同じ大気からは常に同じ鍵が出ることを契約とするため、
//! ハッシュ関数の実装がコンパイラの版で変わることを許せない。

use blitz_math::{メートル, 逆メートル};

const FNVの初期値: u64 = 0xcbf2_9ce4_8422_2325;
const FNVの乗数: u64 = 0x0000_0100_0000_01b3;

/// 値をひとつずつ取り込んで64ビットへ畳み込む器。
pub(super) struct キー畳み込み(u64);

impl キー畳み込み {
    pub(super) fn 新規() -> Self {
        Self(FNVの初期値)
    }

    /// 浮動小数をビット表現で取り込む。`==`による比較を使わずビット表現で畳み込むのは、
    /// 正のゼロと負のゼロが`==`では等しいのに別々の値であり、値の比較で更新判定を行う契約が
    /// 「同じ鍵なら同じ大気」の向きで破れるためである。値の有限性は各値オブジェクトの生成が保証しており、
    /// NaNの複数表現がここへ届くことはない。
    pub(super) fn 実数(&mut self, 値: f32) -> &mut Self {
        for バイト in 値.to_bits().to_le_bytes() {
            self.0 = (self.0 ^ u64::from(バイト)).wrapping_mul(FNVの乗数);
        }
        self
    }

    pub(super) fn 長さ(&mut self, 長さ: メートル) -> &mut Self {
        self.実数(長さ.値())
    }

    pub(super) fn 逆長さ(&mut self, 逆長さ: 逆メートル) -> &mut Self {
        self.実数(逆長さ.値())
    }

    pub(super) fn 三成分(&mut self, 成分: [f32; 3]) -> &mut Self {
        for 値 in 成分 {
            self.実数(値);
        }
        self
    }

    pub(super) fn 値(&self) -> u64 {
        self.0
    }
}
