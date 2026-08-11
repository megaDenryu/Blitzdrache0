//! 統合経路の人間向け要約からDoD判定に必要な整数を集め、判定後の1行要約へ整える。
//! ストリーミング要約の読み取りそのものは`streaming_report`が持ち、ここはvalidation件数を足して要約文にすることだけを行う。

use crate::streaming_report::{ストリーミング要約報告, 先頭整数};
use crate::validation_count::検証層の指摘件数の見出し;

pub(super) struct 計測値 {
    pub(super) 要約: ストリーミング要約報告,
    pub(super) validation件数: u64,
}

pub(super) fn 読み取る(出力: &str) -> Result<計測値, String> {
    let validation行 = 出力
        .lines()
        .map(str::trim)
        .find(|行| 行.starts_with(検証層の指摘件数の見出し))
        .ok_or_else(|| format!("報告に「{検証層の指摘件数の見出し}」の行が無い"))?;
    Ok(計測値 {
        要約: crate::streaming_report::取り出す(出力)?,
        validation件数: 先頭整数(validation行)?,
    })
}

impl 計測値 {
    pub(super) fn 要約文(&self) -> String {
        format!(
            "読込{}件、解除{}件、最大台帳{}件、LOD変更{}フレーム、変更時読込{}件、最終{}段、RAM {} bytes、VRAM {} bytes、validation {}件",
            self.要約.ディスク読込件数,
            self.要約.gpu解除件数,
            self.要約.最大台帳登録数,
            self.要約.lod変更フレーム数,
            self.要約.lod変更時読込開始件数,
            self.要約.最終段種類数,
            self.要約.最大ramバイト数,
            self.要約.最大vramバイト数,
            self.validation件数
        )
    }
}
