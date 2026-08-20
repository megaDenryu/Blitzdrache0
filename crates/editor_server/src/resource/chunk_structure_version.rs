//! 保存済みのチャンク構造を、版ごとの型で受け取って最新の形へ変換する。
//! 1つの型へ互換用の細工を足し続けず、旧版は旧版の型として丸ごと持つ
//! (参照: グローバルCLAUDE.md「シリアライズデータの後方互換は『版ごとの型＋最新への変換』で表す」)。
//! 変換の向きは常に旧版から最新へ一方通行であり、保存は最新の形だけで行う。

use serde::Deserialize;

use super::building::建物の配置;
use super::chunk_road::チャンクの道路;
use super::chunk_structure::チャンク構造;
use super::scatter_settings::散布の設定;

/// 道路を1本だけ持つ旧版のチャンク構造とは、1チャンクに道路が1本しか置けなかった頃の
/// `チャンク/{x}/{z}/構造`のJSONの形のことである。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct 道路を1本だけ持つ旧版のチャンク構造 {
    pub 道路: チャンクの道路,
    pub 建物一覧: Vec<建物の配置>,
    pub 散布: 散布の設定,
}

impl 道路を1本だけ持つ旧版のチャンク構造 {
    pub fn 最新の形へ変換する(self) -> チャンク構造 {
        チャンク構造 {
            道路一覧: vec![self.道路],
            建物一覧: self.建物一覧,
            散布: self.散布,
        }
    }
}

/// 読み込んだチャンク構造の版とは、保存されているJSONがどの版の形で書かれていたかを表す判別のことである。
/// JSONに版番号の欄は無いため、最新の形から順に当てはめ、当てはまった形をその版と決める。
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum 読み込んだチャンク構造の版 {
    最新(チャンク構造),
    道路が1本だけの旧版(道路を1本だけ持つ旧版のチャンク構造),
}

impl 読み込んだチャンク構造の版 {
    pub fn 最新の形へ変換する(self) -> チャンク構造 {
        match self {
            Self::最新(構造) => 構造,
            Self::道路が1本だけの旧版(構造) => 構造.最新の形へ変換する(),
        }
    }
}
