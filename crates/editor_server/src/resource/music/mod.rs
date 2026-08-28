//! 楽曲の型契約。楽曲エディターが打ち込む1曲ぶんを、素データ様式でブラウザとサーバーが共有する。
//!
//! テンポ・音量・ステップ数を素の数値と名前の格で持つのは、この層がserdeとJSONの直列化境界に接するためである。
//! 単位と値域を型で守る数学DDDは、境界の内側にあるブラウザの編集モデルが担う
//! (参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」、`_doc/設計/楽曲エディター.md`)。

mod chord;
mod chord_progression;
mod command;
mod instrument;
mod mixer;
mod music_id;
mod note_rows;
mod pattern;
mod pattern_id;
mod pattern_roster;
mod preset_progression;
mod progression_reference;
mod progression_roster;
mod section;
mod track;
mod track_grid;
mod value_range;
mod version;

use serde::{Deserialize, Serialize};

use super::numeric_check::整数が範囲内であることを確かめる;
use super::text_check::綴りが空でないことを確かめる;
use super::validation_error::資源検証エラー;
use pattern_roster::パターンの名簿;
use progression_roster::進行の名簿;

pub use chord::{和音, 和音の種類};
pub use chord_progression::コード進行;
pub use command::{
    テンポを変える, トラックの楽器を変える, トラックの進行の割り当てを変える, トラックの音量を変える, パターンの打点を全部消す,
    パターンの表示名を変える, パターンの進行を変える, パターンを削除する, パターンを追加する, ミキサー設定を変える, 打ち込みの対象, 打点を消す,
    打点を置く, 曲の節を並べ替える, 曲の節を削除する, 曲の節を変える, 曲の節を追加する, 楽曲の表示名を変える, 楽曲編集コマンド, 独自の進行を保存する,
    独自の進行を削除する, 範囲の打点を消す, 音を伸ばす,
};
pub use instrument::{打楽器の種類, 楽器};
pub use mixer::ミキサー設定;
pub use music_id::楽曲ID;
pub use note_rows::音の並び;
pub use pattern::パターン;
pub use pattern_id::パターンID;
pub use preset_progression::{既定のコード進行, 既定のコード進行一覧};
pub use progression_reference::コード進行参照;
pub use section::曲の節;
pub use track::{トラックの種類, トラック定義};
pub use track_grid::トラックの格子;
pub use value_range::{
    テンポの上限, テンポの下限, パターンのステップ数, 和音の根音の上限, 和音の根音の下限, 和音の続くステップ数の上限, 和音の続くステップ数の下限,
    曲の節の繰り返し回数の上限, 曲の節の繰り返し回数の下限, 遅延のステップ数の上限, 遅延のステップ数の下限, 音量と効果の比の上限,
    音量と効果の比の下限, 音高番号の上限, 音高番号の下限,
};
pub use version::{楽曲の版の移行エラー, 読み込んだ楽曲の版};

pub const 楽曲の現在の形式版: u32 = 1;

/// 楽曲とは、1曲ぶんの打ち込みの内容(テンポ・トラック構成・ミキサー設定・独自のコード進行・パターン・曲構成)を
/// 束ねた、`editor_data/楽曲/<楽曲ID>.json`のJSON1本ぶんの内容のことである。テンポは1分あたりの拍の数である。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 楽曲 {
    pub 形式版: u32,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: 楽曲ID,
    pub 表示名: String,
    pub テンポ: u32,
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
        if self.形式版 != 楽曲の現在の形式版 {
            return Err(資源検証エラー::未対応の形式版 {
                フィールド名: "楽曲",
                実際: self.形式版,
                対応: 楽曲の現在の形式版,
            });
        }
        綴りが空でないことを確かめる("楽曲.表示名", &self.表示名)?;
        整数が範囲内であることを確かめる("楽曲.テンポ", i64::from(self.テンポ), i64::from(テンポの下限), i64::from(テンポの上限))?;
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
