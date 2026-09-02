//! 布の1刻みのGPU時間を、パス別GPU時間の表から読む工程。受け取るのは終了時報告、返すのは布のパスごとのp50とその合計である。
//! 表の行の読み方は`xpbd_solver_bench/parse.rs`と同じ(「平均 (p50 X / p95 Y / 標本N)」)であり、布のパスは名前が「布」で始まる。
//! 頂点生成は描画機会ごとの工程であって刻みの工程ではないが、既存の値(約0.051ミリ秒)が全部の布のパスの和であるため同じ範囲で足す。

use crate::acceptance::{判定の名前, 検収エラー, 終了時報告};
use crate::report_heading::報告の見出し;
use crate::report_line_key::{報告の行の鍵, 鍵に値が結ばれる形};

const 表の見出し: 報告の見出し = 報告の見出し::定数から生成する("パス別GPU時間");
const 中央値の鍵: 報告の行の鍵 = 報告の行の鍵::定数から生成する("p50");
const 布のパスの接頭辞: &str = "布";

pub(super) struct 布の一刻みのGPU時間 {
    pub(super) パス別のp50ミリ秒: Vec<(String, f64)>,
    pub(super) 合計のp50ミリ秒: f64,
}

impl 布の一刻みのGPU時間 {
    pub(super) fn 要約(&self) -> String {
        let 内訳: Vec<String> = self.パス別のp50ミリ秒.iter().map(|(名前, 値)| format!("{名前}{値:.4}")).collect();
        format!("布のパスのp50の合計は{:.4}ミリ秒({})", self.合計のp50ミリ秒, 内訳.join("・"))
    }
}

pub(super) fn 布の一刻みを読む(報告: &終了時報告) -> Result<布の一刻みのGPU時間, 検収エラー> {
    let 表 = 報告.見出しの区画(表の見出し)?;
    let mut パス別のp50ミリ秒 = Vec::new();
    for 行 in 表.行一覧() {
        let 原文 = 行.原文().trim_start();
        let Some(名前) = 原文.strip_prefix(布のパスの接頭辞).and_then(|残り| 残り.split(':').next()) else {
            continue;
        };
        let p50 = 行.鍵に結ばれた数(&中央値の鍵, 鍵に値が結ばれる形::この綴りで終わる語の次が値である)?;
        パス別のp50ミリ秒.push((format!("{布のパスの接頭辞}{名前}"), p50));
    }
    if パス別のp50ミリ秒.is_empty() {
        return Err(
            判定の名前::定数から生成する("パス別GPU時間の表の中の布のパス(布ありの実行なら1本以上ある)")
                .あるはずのものが無い破れ()
                .into(),
        );
    }
    let 合計のp50ミリ秒 = パス別のp50ミリ秒.iter().map(|(_, 値)| *値).sum();
    Ok(布の一刻みのGPU時間 {
        パス別のp50ミリ秒,
        合計のp50ミリ秒,
    })
}
