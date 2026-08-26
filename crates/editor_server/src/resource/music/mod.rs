//! 楽曲の型契約。楽曲エディターが打ち込む1曲ぶんを、素データ様式でブラウザとサーバーが共有する。
//!
//! 拍毎分・音量・ステップ数を素の数値と名前の格で持つのは、この層がserdeとJSONの直列化境界に接するためである。
//! 単位と値域を型で守る数学DDDは、境界の内側にあるブラウザの編集モデルが担う
//! (参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」、`_doc/設計/楽曲エディター.md`)。

mod chord;
mod chord_progression;
mod instrument;
mod mixer;
mod music_id;
mod note_rows;
mod pattern;
mod pattern_id;
mod pattern_roster;
mod progression_reference;
mod progression_roster;
mod section;
mod track;
mod track_grid;

use serde::{Deserialize, Serialize};

use super::numeric_check::整数が範囲内であることを確かめる;
use super::validation_error::資源検証エラー;
use pattern_roster::パターンの名簿;
use progression_roster::進行の名簿;

pub use chord::{和音, 和音の種類};
pub use chord_progression::{コード進行, 既定の進行の識別子一覧};
pub use instrument::{打楽器の種類, 楽器};
pub use mixer::ミキサー設定;
pub use music_id::楽曲ID;
pub use note_rows::音の並び;
pub use pattern::パターン;
pub use pattern_id::パターンID;
pub use progression_reference::コード進行参照;
pub use section::曲の節;
pub use track::{トラックの種類, トラック定義};
pub use track_grid::{トラックの格子, パターンのステップ数};

const 拍毎分の下限: u32 = 40;
const 拍毎分の上限: u32 = 300;

/// 楽曲とは、1曲ぶんの打ち込みの内容(拍毎分・トラック構成・ミキサー設定・独自のコード進行・パターン・曲構成)を
/// 束ねた、`editor_data/楽曲/<楽曲ID>.json`のJSON1本ぶんの内容のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 楽曲 {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: 楽曲ID,
    pub 表示名: String,
    pub 拍毎分: u32,
    pub トラック構成: Vec<トラック定義>,
    pub ミキサー設定: ミキサー設定,
    pub 独自進行一覧: Vec<コード進行>,
    pub パターン一覧: Vec<パターン>,
    pub 曲構成: Vec<曲の節>,
}

impl 楽曲 {
    /// 名簿を先に組み立てるのは、名簿の組み立て自体が名前と名乗りの重複を拒む検査だからである。
    /// 名簿が成り立ってはじめて、参照の解決が1件に定まる問いになる。
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        整数が範囲内であることを確かめる("楽曲.拍毎分", i64::from(self.拍毎分), i64::from(拍毎分の下限), i64::from(拍毎分の上限))?;
        self.ミキサー設定.検証する()?;
        let 進行の名簿 = 進行の名簿::独自進行一覧から組み立てる(&self.独自進行一覧)?;
        self.トラック構成を確かめる(&進行の名簿)?;
        let パターンの名簿 = パターンの名簿::パターン一覧から組み立てる(&self.パターン一覧)?;
        for パターン in &self.パターン一覧 {
            パターン.検証する(&self.トラック構成, &進行の名簿)?;
        }
        for 節 in &self.曲構成 {
            節.検証する(&パターンの名簿)?;
        }
        Ok(())
    }

    fn トラック構成を確かめる(&self, 進行の名簿: &進行の名簿<'_>) -> Result<(), 資源検証エラー> {
        if self.トラック構成.is_empty() {
            return Err(資源検証エラー::最小件数を下回る {
                フィールド名: "楽曲.トラック構成",
                値: 0,
            });
        }
        for トラック in &self.トラック構成 {
            トラック.検証する(進行の名簿)?;
        }
        Ok(())
    }
}
