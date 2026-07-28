//! 必要性と非同期処理段階をチャンク座標ごとに一意管理する台帳。

mod eviction;
mod eviction_victim;
mod transitions;
mod update;
mod usage;

use std::collections::HashMap;

use crate::チャンク座標;

use super::{chunk_request::チャンク要求, chunk_state::チャンク状態, memory_amount::ストリーミングメモリ量};

#[derive(Debug)]
pub struct チャンク台帳 {
    登録一覧: HashMap<チャンク座標, チャンク記録>,
}

#[derive(Debug, Clone, Copy)]
struct チャンク記録 {
    要求: チャンク要求,
    /// 注意: `解除待ち`へ進んだ記録は前の段階へ戻らない。レンダラーの描画束は解除を予約した時点で取り消せないため、
    /// 台帳だけが常駐へ戻ると描かれないまま常駐として数え、実破棄の報告が状態遷移不正になる。
    状態: チャンク状態,
    必要: bool,
    /// 注意: `ramバイト数`は読込中までは読込前の見積であり、準備完了の反映で実測へ置き換わる。`vramバイト数`は見積である。
    /// 現在使用量はこの量と状態から導出し、別フィールドとして保管しない。
    メモリ量: ストリーミングメモリ量,
}

impl チャンク台帳 {
    pub fn 空を作る() -> Self {
        Self {
            登録一覧: HashMap::new()
        }
    }

    pub fn 状態を引く(&self, 座標: チャンク座標) -> Option<チャンク状態> {
        self.登録一覧.get(&座標).map(|記録| 記録.状態)
    }

    /// 記録が保持している量。RAMは準備完了以降は実測、VRAMは常に見積である。転送量の集計がGPUへ載せた量を知るために引く。
    pub fn メモリ量を引く(&self, 座標: チャンク座標) -> Option<ストリーミングメモリ量> {
        self.登録一覧.get(&座標).map(|記録| 記録.メモリ量)
    }

    pub fn 登録数(&self) -> usize {
        self.登録一覧.len()
    }

    /// 常駐している座標を受け皿へ集める。受け皿を使い回す形にするのは、毎フレームの走査でヒープを再確保しないためである。
    /// 並びはハッシュ表の走査順であり定まらないため、順序に意味を持たせる呼出し側が整列させる。
    pub fn 常駐座標を集める(&self, 受け皿: &mut Vec<チャンク座標>) {
        受け皿.clear();
        for (座標, 記録) in &self.登録一覧 {
            if 記録.状態 == チャンク状態::常駐 {
                受け皿.push(*座標);
            }
        }
    }
}
