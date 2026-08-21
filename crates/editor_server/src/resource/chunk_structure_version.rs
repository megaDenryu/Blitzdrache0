//! 保存済みのチャンク構造を版ごとの型で受け、最新の建物定義IDを持つ形へ一方向に変換する。

mod legacy_building_placement;

use serde::Deserialize;
use thiserror::Error;

use self::legacy_building_placement::{旧版の建物一覧を最新の形へ変換する, 種別を持つ旧版の建物配置};
use super::building::建物の配置;
use super::chunk_road::チャンクの道路;
use super::chunk_structure::チャンク構造;
use super::scatter_settings::散布の設定;

/// 散布の個体一覧を持たない版。散布の設定だけを保存していた頃の形であり、個体一覧は空として読む。
/// 空で読むのは、個体の列が設定から導かれる派生であり、次の保存で編集モデルが導き直して埋まるためである。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct 散布の個体一覧を持たない旧版のチャンク構造 {
    道路一覧: Vec<チャンクの道路>,
    建物一覧: Vec<建物の配置>,
    散布: 散布の設定,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct 建物種別を持つ旧版のチャンク構造 {
    道路一覧: Vec<チャンクの道路>,
    建物一覧: Vec<種別を持つ旧版の建物配置>,
    散布: 散布の設定,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct 道路を1本だけ持つ最旧版のチャンク構造 {
    道路: チャンクの道路,
    建物一覧: Vec<種別を持つ旧版の建物配置>,
    散布: 散布の設定,
}

#[derive(Debug, Error)]
pub enum チャンク構造移行エラー {
    #[error("旧版の建物{建物識別子}（種別: {旧種別}）に対応する建物定義が無い")]
    対応する建物定義が無い { 建物識別子: String, 旧種別: String },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum 読み込んだチャンク構造の版 {
    最新(チャンク構造),
    散布の個体一覧を持たない旧版(散布の個体一覧を持たない旧版のチャンク構造),
    建物種別を持つ旧版(建物種別を持つ旧版のチャンク構造),
    道路が1本だけの最旧版(道路を1本だけ持つ最旧版のチャンク構造),
}

impl 読み込んだチャンク構造の版 {
    pub fn 最新の形へ変換する(self) -> Result<チャンク構造, チャンク構造移行エラー> {
        match self {
            Self::最新(構造) => Ok(構造),
            Self::散布の個体一覧を持たない旧版(構造) => Ok(チャンク構造 {
                道路一覧: 構造.道路一覧,
                建物一覧: 構造.建物一覧,
                散布: 構造.散布,
                散布の個体一覧: Vec::new(),
            }),
            Self::建物種別を持つ旧版(構造) => Ok(チャンク構造 {
                道路一覧: 構造.道路一覧,
                建物一覧: 旧版の建物一覧を最新の形へ変換する(構造.建物一覧)?,
                散布: 構造.散布,
                散布の個体一覧: Vec::new(),
            }),
            Self::道路が1本だけの最旧版(構造) => Ok(チャンク構造 {
                道路一覧: vec![構造.道路],
                建物一覧: 旧版の建物一覧を最新の形へ変換する(構造.建物一覧)?,
                散布: 構造.散布,
                散布の個体一覧: Vec::new(),
            }),
        }
    }
}
