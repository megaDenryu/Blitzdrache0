//! 材質の3軸を1つにまとめた、材質変種キーの導出の入力。担当するのは、3つの軸が必ず同じ材質から同時に決まることと、
//! そのうち材質特徴集合だけがGPUの材質レコードへも運ばれることを1つの型で示すことである。
//!
//! 3つを別々の引数で配らないのは、軸が増えたときに配り忘れた呼び出し元が黙って古い軸だけで導出してしまうためである。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「材質の3軸」

use super::capability_table;
use super::error::材質能力エラー;
use super::shading_model::シェーディングモデル種別;
use super::surface_state::表面描画状態;
use super::variant_key::材質変種キー;
use crate::vulkan::material_table::材質特徴集合;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 材質の3軸 {
    モデル: シェーディングモデル種別,
    特徴集合: 材質特徴集合,
    表面描画状態: 表面描画状態,
}

impl 材質の3軸 {
    pub(crate) const fn 生成する(
        モデル: シェーディングモデル種別, 特徴集合: 材質特徴集合, 表面描画状態: 表面描画状態
    ) -> Self {
        Self {
            モデル,
            特徴集合,
            表面描画状態,
        }
    }

    /// GPUの材質レコードへ運ぶ軸。パイプラインを変えないため変種キーには現れないが、画素段が係数へ掛けるかどうかを
    /// この値が決めるため、レコードの側では捨てられない。
    pub(crate) const fn 特徴集合(self) -> 材質特徴集合 {
        self.特徴集合
    }

    /// 3軸を正規化済みの材質変種キーへ写す唯一の工程。特徴集合はこの正規化で必ず畳まれる軸であり、
    /// 8通りのどれも同じキーへ写る(理由は`capability_table`の冒頭にある)。
    pub(crate) fn 変種キーへ正規化する(self) -> Result<材質変種キー, 材質能力エラー> {
        capability_table::引く(self.モデル, self.表面描画状態)
    }
}
