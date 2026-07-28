//! 地形1チャンクの詳細段ごとのメッシュ列。段番号の昇順に並び、先頭が最詳細である。
//! 先頭を別引数で受けるため空の列を作れず、どの段を要求されても描く幾何が必ず存在する。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「地形の表現」

use super::mesh_data::メッシュデータ;

/// 地形の詳細段が満たすべき条件を破ったときの失敗。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum 地形LODメッシュ群エラー {
    #[error("地形の詳細段{段番号}にスキン頂点属性が含まれている")]
    段にスキン頂点属性がある { 段番号: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct 地形LODメッシュ群 {
    段一覧: Vec<メッシュデータ>,
}

impl 地形LODメッシュ群 {
    /// 地形はスキニングの対象にせず、`描画形状::スキン頂点属性一覧`は地形なら無条件で`None`を返す。
    /// 検査をこの境界に置かないと、外部から与えられたバイト列の段にスキン頂点属性が入っていても誰も拒否せず、属性が無言で捨てられる。
    pub fn 生成する(
        最詳細段: メッシュデータ, より粗い段一覧: Vec<メッシュデータ>
    ) -> Result<Self, 地形LODメッシュ群エラー> {
        let mut 段一覧 = Vec::with_capacity(1 + より粗い段一覧.len());
        段一覧.push(最詳細段);
        段一覧.extend(より粗い段一覧);
        for (段番号, 段) in 段一覧.iter().enumerate() {
            if 段.スキン頂点属性一覧.is_some() {
                return Err(地形LODメッシュ群エラー::段にスキン頂点属性がある { 段番号 });
            }
        }
        Ok(Self { 段一覧 })
    }

    pub fn 段一覧(&self) -> &[メッシュデータ] {
        &self.段一覧
    }

    pub fn 段数(&self) -> usize {
        self.段一覧.len()
    }

    pub fn 最詳細メッシュ(&self) -> &メッシュデータ {
        match self.段一覧.first() {
            Some(メッシュ) => メッシュ,
            None => panic!("地形LODメッシュ群は1段以上を持つ不変条件に違反した"),
        }
    }
}
