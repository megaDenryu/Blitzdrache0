//! 1フレームの調停で実際に起きた読込・準備完了・解除と、そのフレームの予算判定。
//! いずれもそのフレームで観測できた事実だけを持ち、将来の見込みや未着手の予定は持たない。

use crate::チャンク座標;

use super::{memory_amount::ストリーミングメモリ量, memory_result::予算判定};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ストリーミング進行 {
    pub(super) 読込開始一覧: Vec<チャンク座標>,
    pub(super) 準備完了一覧: Vec<チャンク座標>,
    pub(super) cpuデータ破棄一覧: Vec<チャンク座標>,
    pub(super) gpu資源解除待ち一覧: Vec<チャンク座標>,
    pub(super) 使用量: ストリーミングメモリ量,
    pub(super) 台帳登録数: usize,
    pub(super) 判定: 予算判定,
}

impl ストリーミング進行 {
    /// ディスク読込を開始した順。中心優先の順序をそのまま保つ。
    pub fn 読込開始一覧(&self) -> &[チャンク座標] {
        &self.読込開始一覧
    }

    /// 読込が完了してCPU側に載った順。
    pub fn 準備完了一覧(&self) -> &[チャンク座標] {
        &self.準備完了一覧
    }

    /// 必要でなくなったためCPUデータを捨てた座標。
    pub fn cpuデータ破棄一覧(&self) -> &[チャンク座標] {
        &self.cpuデータ破棄一覧
    }

    /// GPU資源の解除待ちへ移した座標。呼出し側はGPU資源を解放してから`gpu資源の解除を報告する`を呼ぶ。
    pub fn gpu資源解除待ち一覧(&self) -> &[チャンク座標] {
        &self.gpu資源解除待ち一覧
    }

    /// このフレームの反映と回収を終えた時点で台帳が導出したRAM・VRAM使用量。
    pub fn 使用量(&self) -> ストリーミングメモリ量 {
        self.使用量
    }

    /// 台帳が管理しているチャンクの件数。読込待ちから解除待ちまでのすべての段階を含む。
    pub fn 台帳登録数(&self) -> usize {
        self.台帳登録数
    }

    pub fn 判定(&self) -> 予算判定 {
        self.判定
    }

    pub fn 事象があるか(&self) -> bool {
        !self.読込開始一覧.is_empty() || !self.準備完了一覧.is_empty() || !self.cpuデータ破棄一覧.is_empty() || !self.gpu資源解除待ち一覧.is_empty()
    }
}
