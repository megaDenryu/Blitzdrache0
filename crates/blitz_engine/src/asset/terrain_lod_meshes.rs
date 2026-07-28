//! 地形1チャンクの詳細段ごとのメッシュ列。段番号の昇順に並び、先頭が最詳細である。
//! 先頭を別引数で受けるため空の列を作れず、どの段を要求されても描く幾何が必ず存在する。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「地形の表現」

use blitz_render::地形詳細段;

use super::mesh_data::メッシュデータ;

#[derive(Debug, Clone, PartialEq)]
pub struct 地形LODメッシュ群 {
    段一覧: Vec<メッシュデータ>,
}

impl 地形LODメッシュ群 {
    pub fn 生成する(最詳細段: メッシュデータ, より粗い段一覧: Vec<メッシュデータ>) -> Self {
        let mut 段一覧 = Vec::with_capacity(1 + より粗い段一覧.len());
        段一覧.push(最詳細段);
        段一覧.extend(より粗い段一覧);
        Self { 段一覧 }
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

    /// 要求された段のメッシュ。焼かれた段数を超える要求は最も粗い段を返す。
    /// 距離が最も粗い段の閾値より遠いという意味であり、そのとき最も粗い段を描くのが要求どおりの結果だからである。
    pub fn 段を引く(&self, 段: 地形詳細段) -> &メッシュデータ {
        match self.段一覧.get(段.添字()).or_else(|| self.段一覧.last()) {
            Some(メッシュ) => メッシュ,
            None => panic!("地形LODメッシュ群は1段以上を持つ不変条件に違反した"),
        }
    }
}
