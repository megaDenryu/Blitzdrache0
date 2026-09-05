//! 形式版1の楽曲とパターン。パターンが小節数の欄を持つ前の形であり、既定の小節数(2)を名指しで置いて
//! 現在の形へ変換する(参照: `_doc/設計/楽曲エディター.md`「判断16」)。

use serde::Deserialize;

use super::super::chord_progression::コード進行;
use super::super::mixer::ミキサー設定;
use super::super::music_id::楽曲ID;
use super::super::pattern::パターン;
use super::super::pattern_id::パターンID;
use super::super::progression_reference::コード進行参照;
use super::super::section::曲の節;
use super::super::track::トラック定義;
use super::super::track_grid::トラックの格子;
use super::super::value_range::新しいパターンの既定の小節数;
use super::super::{楽曲, 楽曲の現在の形式版};

/// 形式版1のパターンとは、パターンが小節数の欄を持つ前の形のことである。32ステップの格子は小節数2として移行する。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct 形式版1のパターン {
    pub 名乗り: パターンID,
    pub 表示名: String,
    pub 進行の参照: コード進行参照,
    pub 格子: Vec<トラックの格子>,
}

impl 形式版1のパターン {
    /// 小節数の欄だけが無い版であるため、既定の小節数を名指しで置く。打点の位置と長さは変えない。
    pub fn 現在の形へ変換する(self) -> パターン {
        パターン {
            名乗り: self.名乗り,
            表示名: self.表示名,
            小節数: 新しいパターンの既定の小節数,
            進行の参照: self.進行の参照,
            格子: self.格子,
        }
    }
}

/// 形式版1の楽曲とは、パターンが小節数の欄を持つ前に保存された`楽曲/<楽曲ID>.json`の形のことである。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct 形式版1の楽曲 {
    pub 形式版: u32,
    pub 名乗り: 楽曲ID,
    pub 表示名: String,
    pub テンポ: u32,
    pub トラック構成: Vec<トラック定義>,
    pub ミキサー設定: ミキサー設定,
    pub 独自進行一覧: Vec<コード進行>,
    pub パターン一覧: Vec<形式版1のパターン>,
    pub 曲構成: Vec<曲の節>,
}

impl 形式版1の楽曲 {
    /// 形式版の数値はそのまま読み替えず捨て、現在の形式版を名指しで置く。各パターンは形式版1の変換へ委ねる。
    pub fn 現在の形へ変換する(self) -> 楽曲 {
        楽曲 {
            形式版: 楽曲の現在の形式版,
            名乗り: self.名乗り,
            表示名: self.表示名,
            テンポ: self.テンポ,
            トラック構成: self.トラック構成,
            ミキサー設定: self.ミキサー設定,
            独自進行一覧: self.独自進行一覧,
            パターン一覧: self.パターン一覧.into_iter().map(形式版1のパターン::現在の形へ変換する).collect(),
            曲構成: self.曲構成,
        }
    }
}
