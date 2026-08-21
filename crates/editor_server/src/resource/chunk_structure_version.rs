//! 保存済みのチャンク構造を版ごとの型で受け、最新の建物定義IDを持つ形へ一方向に変換する。

use serde::Deserialize;
use thiserror::Error;

use super::building::建物の配置;
use super::chunk_road::チャンクの道路;
use super::chunk_structure::チャンク構造;
use super::position::位置3次元;
use super::scatter_settings::散布の設定;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum 旧版の建物種別 {
    家屋,
    塔,
    宝箱,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct 種別を持つ旧版の建物配置 {
    識別子: String,
    種別: 旧版の建物種別,
    位置: 位置3次元,
    向きラジアン: f64,
    基礎半径メートル: f64,
    なじみ半径メートル: f64,
}

impl 種別を持つ旧版の建物配置 {
    fn 最新の形へ変換する(self) -> Result<建物の配置, チャンク構造移行エラー> {
        let 建物定義_id = match self.種別 {
            旧版の建物種別::家屋 => "frame_house_one_bay",
            旧版の建物種別::塔 | 旧版の建物種別::宝箱 => {
                return Err(チャンク構造移行エラー::対応する建物定義が無い {
                    建物識別子: self.識別子,
                    旧種別: format!("{:?}", self.種別),
                });
            }
        };
        Ok(建物の配置 {
            識別子: self.識別子,
            建物定義ID: 建物定義_id.to_string(),
            位置: self.位置,
            向きラジアン: self.向きラジアン,
            基礎半径メートル: self.基礎半径メートル,
            なじみ半径メートル: self.なじみ半径メートル,
        })
    }
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
    建物種別を持つ旧版(建物種別を持つ旧版のチャンク構造),
    道路が1本だけの最旧版(道路を1本だけ持つ最旧版のチャンク構造),
}

impl 読み込んだチャンク構造の版 {
    pub fn 最新の形へ変換する(self) -> Result<チャンク構造, チャンク構造移行エラー> {
        match self {
            Self::最新(構造) => Ok(構造),
            Self::建物種別を持つ旧版(構造) => Ok(チャンク構造 {
                道路一覧: 構造.道路一覧,
                建物一覧: 建物一覧を変換する(構造.建物一覧)?,
                散布: 構造.散布,
            }),
            Self::道路が1本だけの最旧版(構造) => Ok(チャンク構造 {
                道路一覧: vec![構造.道路],
                建物一覧: 建物一覧を変換する(構造.建物一覧)?,
                散布: 構造.散布,
            }),
        }
    }
}
fn 建物一覧を変換する(
    旧一覧: Vec<種別を持つ旧版の建物配置>
) -> Result<Vec<建物の配置>, チャンク構造移行エラー> {
    旧一覧.into_iter().map(種別を持つ旧版の建物配置::最新の形へ変換する).collect()
}
