//! 曲げ拘束の彩色: 同じ点を同時に触らない曲げ拘束の集合へ拘束を色分けする前計算(判断7の採用規則1。固定トポロジーの曲げはグラフ彩色)。
//! 貪欲法(拘束を並びの順に見て、4点がまだ使っていない最小の色を与える)であり、決定的である。曲げ拘束は4点を触るため距離拘束の色とは別の区間になり、
//! GPUは距離拘束の色の後ろに曲げの色を積む(`_doc/設計/XPBD共通拘束基盤.md`「判断8」)。色ごとに連続した並びへ組み替えて持つのは、GPUが色ごとに1回のディスパッチで区間を処理するためである。

use super::bending_batch::{曲げ拘束のバッチ, 添字付き曲げ拘束};

/// 曲げ拘束の並びの1本を指す添字。距離拘束の拘束添字と別の型にするのは、乗数バッファの区間の開始と取り違えを型で止めるためである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 曲げ拘束添字(u32);

impl 曲げ拘束添字 {
    /// GPUのプッシュ定数へ書く境界向けの生値。
    pub fn 値(&self) -> u32 {
        self.0
    }

    pub fn 配列添字(&self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("曲げ拘束添字がusizeに収まらない: {}", self.0))
    }

    fn 配列添字から生成する(添字: usize) -> Self {
        Self(u32::try_from(添字).unwrap_or_else(|_| panic!("曲げ拘束の数がu32に収まらない: {添字}")))
    }
}

/// 色1つが占める、並べ替えた曲げ拘束一覧の中の連続した区間。GPUのディスパッチ1回分である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 曲げの色の区間 {
    pub 開始: 曲げ拘束添字,
    pub 本数: u32,
}

/// 彩色の結果。曲げ拘束を色ごとに連続するよう並べ替えた一覧と、色ごとの区間を持つ。
#[derive(Debug, Clone, PartialEq)]
pub struct 曲げ拘束の彩色 {
    並べ替えた拘束一覧: Vec<添字付き曲げ拘束>,
    色の区間一覧: Vec<曲げの色の区間>,
}

impl 曲げ拘束の彩色 {
    pub fn 生成する(バッチ: &曲げ拘束のバッチ) -> Self {
        let 色一覧 = バッチ.拘束へ貪欲に色を割り当てる();
        let 色の数 = 色一覧.iter().map(|色| 色 + 1).max().unwrap_or(0);
        let mut 並べ替えた = Vec::with_capacity(バッチ.拘束の数());
        let mut 色の区間一覧 = Vec::with_capacity(色の数);
        for 色 in 0..色の数 {
            let 開始 = 曲げ拘束添字::配列添字から生成する(並べ替えた.len());
            for (拘束, 割り当て) in バッチ.拘束一覧().iter().zip(&色一覧) {
                if *割り当て == 色 {
                    並べ替えた.push(*拘束);
                }
            }
            let 本数 = u32::try_from(並べ替えた.len() - 開始.配列添字()).unwrap_or_else(|_| panic!("色の本数がu32に収まらない"));
            色の区間一覧.push(曲げの色の区間 { 開始, 本数 });
        }
        Self {
            並べ替えた拘束一覧: 並べ替えた,
            色の区間一覧,
        }
    }

    /// 色ごとに連続した並びへ組み替えた拘束一覧。GPUの曲げ拘束バッファと布の参照計算がこの並びを使う。
    pub fn 拘束一覧(&self) -> &[添字付き曲げ拘束] {
        &self.並べ替えた拘束一覧
    }

    pub fn 拘束の数(&self) -> usize {
        self.並べ替えた拘束一覧.len()
    }

    pub fn 色の区間一覧(&self) -> &[曲げの色の区間] {
        &self.色の区間一覧
    }

    pub fn 色の数(&self) -> usize {
        self.色の区間一覧.len()
    }
}
