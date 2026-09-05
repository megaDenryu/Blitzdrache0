//! 楽曲1件の型契約と、その全体の検証。楽曲とは、`editor_data/楽曲/<楽曲ID>.json`のJSON1本ぶんの内容のことである。
//!
//! 束ねる側をこのファイルへ置き、束ねられる側(和音・パターン・曲の節・トラック定義)を兄弟のファイルへ置くのは、
//! 全体の検証がどの順で部分の検証を呼ぶかという判断を、部分の定義から切り離して読めるようにするためである。

use serde::{Deserialize, Serialize};

use super::super::numeric_check::整数が範囲内であることを確かめる;
use super::super::text_check::綴りが空でないことを確かめる;
use super::super::validation_error::資源検証エラー;
use super::chord_progression::コード進行;
use super::mixer::ミキサー設定;
use super::music_id::楽曲ID;
use super::pattern::パターン;
use super::pattern_roster::パターンの名簿;
use super::progression_roster::進行の名簿;
use super::section::曲の節;
use super::track::トラック定義;
use super::value_range::{テンポの上限, テンポの下限};

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
