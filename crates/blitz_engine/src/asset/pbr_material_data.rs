//! glTFのPBR metallic-roughnessパラメータ。

mod features;
mod features_error;
#[cfg(test)]
mod features_tests;

use super::texture_storage::格納済みテクスチャ;

pub use features::材質特徴集合;
pub use features_error::材質特徴集合エラー;

/// テクスチャは復号済みの画素ではなく、実行時形式が運ぶ形の`格納済みテクスチャ`で持つ。
/// 格納形式ごとに画素の並びが違い、ブロック圧縮の値をCPUで画素として読む経路は存在しないためである。
#[derive(Debug, Clone, PartialEq)]
pub struct 金属粗さPBRデータ {
    pub ベースカラー: Option<格納済みテクスチャ>,
    /// glTF規約ではG成分が粗さ、B成分が金属度を表す。
    pub 金属粗さ: Option<格納済みテクスチャ>,
    pub 法線マップ: Option<格納済みテクスチャ>,
    pub ベースカラー係数: [f32; 4],
    pub 金属度係数: f32,
    pub 粗さ係数: f32,
}

impl 金属粗さPBRデータ {
    /// テクスチャのOption列から特徴集合を導出する正本。アセットコンパイラも旧版からの変換もこの1つを通るため、
    /// 材質データと特徴ビットが食い違った状態を作れる経路が存在しない。
    pub fn 特徴集合(&self) -> 材質特徴集合 {
        材質特徴集合::テクスチャの有無から導出する(
            self.ベースカラー.is_some(),
            self.金属粗さ.is_some(),
            self.法線マップ.is_some(),
        )
    }

    /// 直列化された宣言と、実際に読めたテクスチャの有無が両方向で一致することを確かめる。
    /// ビット有りでデータ無しの場合と、ビット無しでデータ有りの場合の両方をここで拒む。
    pub fn 宣言された特徴集合と照合する(&self, 宣言: 材質特徴集合) -> Result<(), 材質特徴集合エラー> {
        let 導出 = self.特徴集合();
        if 宣言 == 導出 {
            return Ok(());
        }
        Err(材質特徴集合エラー::データと不一致 {
            宣言: 宣言.生値(),
            導出: 導出.生値(),
        })
    }
}
