//! チャンク座標: XZ平面の格子上の位置であり、それ自体が世界のチャンクの識別子である。
//! 座標から独立した番号空間を持たないため、描画対象の所有チャンクと格子の座標が別空間へ分裂しない。
//! 参照: `_doc/設計/チャンクストリーミング.md`「チャンク格子」

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct チャンク座標 {
    x: i32,
    z: i32,
}

impl チャンク座標 {
    /// 座標の2成分からのみ作れるため、どのチャンクも指さない識別子を作る経路が存在しない。
    pub const fn 生成する(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// 永続化境界: Xの32ビット表現を上位、Zの32ビット表現を下位へ置いたu64を返す。`番号から復元する`と可逆である。
    pub fn 番号を返す(self) -> u64 {
        let [x最上位, x第2, x第3, x最下位] = self.x.to_be_bytes();
        let [z最上位, z第2, z第3, z最下位] = self.z.to_be_bytes();
        u64::from_be_bytes([x最上位, x第2, x第3, x最下位, z最上位, z第2, z第3, z最下位])
    }

    /// 永続化境界: `番号を返す`が書いたu64を座標へ戻す。すべてのu64がちょうど1つの座標へ対応するため範囲エラーが存在しない。
    pub fn 番号から復元する(番号: u64) -> Self {
        let [x最上位, x第2, x第3, x最下位, z最上位, z第2, z第3, z最下位] = 番号.to_be_bytes();
        Self {
            x: i32::from_be_bytes([x最上位, x第2, x第3, x最下位]),
            z: i32::from_be_bytes([z最上位, z第2, z第3, z最下位]),
        }
    }

    pub fn x(self) -> i32 {
        self.x
    }

    pub fn z(self) -> i32 {
        self.z
    }
}
