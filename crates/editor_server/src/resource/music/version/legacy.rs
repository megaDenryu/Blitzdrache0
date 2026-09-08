//! 形式版を持たない旧版の楽曲。楽曲の資源に形式版の欄を足す前に保存された形であり、現在の形式版を
//! 名指しで置いてから形式版1の変換へ経由して現在の形へ変換する。

use serde::Deserialize;

use super::super::chord_progression::コード進行;
use super::super::mixer::ミキサー設定;
use super::super::music_id::楽曲ID;
use super::super::section::曲の節;
use super::super::track::トラック定義;
use super::super::{楽曲, 楽曲の現在の形式版};
use super::format_1::{形式版1のパターン, 形式版1の楽曲};

/// 形式版を持たない旧版の楽曲とは、楽曲の資源に形式版の欄を足す前に保存された`楽曲/<楽曲ID>.json`の形のことである。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct 形式版を持たない旧版の楽曲 {
    pub 名乗り: 楽曲ID,
    pub 表示名: String,
    pub テンポ: u32,
    pub トラック構成: Vec<トラック定義>,
    pub ミキサー設定: ミキサー設定,
    pub 独自進行一覧: Vec<コード進行>,
    pub パターン一覧: Vec<形式版1のパターン>,
    pub 曲構成: Vec<曲の節>,
}

impl 形式版を持たない旧版の楽曲 {
    /// 形式版の欄だけが無い版であるため、現在の形式版を名指しで置いてから形式版1の変換へ経由する。
    pub fn 現在の形へ変換する(self) -> 楽曲 {
        形式版1の楽曲 {
            形式版: 楽曲の現在の形式版,
            名乗り: self.名乗り,
            表示名: self.表示名,
            テンポ: self.テンポ,
            トラック構成: self.トラック構成,
            ミキサー設定: self.ミキサー設定,
            独自進行一覧: self.独自進行一覧,
            パターン一覧: self.パターン一覧,
            曲構成: self.曲構成,
        }
        .現在の形へ変換する()
    }
}
