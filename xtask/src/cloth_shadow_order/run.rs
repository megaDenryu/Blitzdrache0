//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「条件と出力名を受け取り、最終フレームの画素を返す」ことである。
//!
//! quadシーンを`--object-count 2`で描くと、同じ板が別々の配置変換で左右に2つ並ぶ。並びが左右に離れているため、
//! 走査順を入れ替えても幾何は1画素も変わらない。板が重なる構図を選ぶと、同一平面の奥行き競合が走査順で決着を変え、
//! 布とは無関係な差が出てしまう。
//!
//! 布は板の手前に垂れ、方向光が板の上へ布の影を落とす。この影が走査順で動かないことがこの検収の対象である。
//! ポスト処理を外すのは、光のにじみが布シミュレーションのわずかな揺れを画面全体へ広げるためである。
//! フレーム数を3に取るのは、布の自己衝突がGPUの原子的加算の順序に依存し、歩進を重ねるほど実行ごとの差が育つためである。
//! 実測では3歩進までは同一条件2回の読み戻しがバイト一致し、4歩進から差が出はじめる。布は初期形状のまま垂れており、影は最初の歩進から板に出る。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/runtime_assets";
const シーン名: &str = "quad";
const 描画対象数: &str = "2";
const フレーム数: &str = "3";

#[derive(Clone, Copy)]
pub(super) enum 条件 {
    /// 布ありでシーンデータの並び順に描画対象を載せる。比べる基準になる条件である。
    布あり読込順,
    /// 布ありで描画対象の並びだけを逆にする。読込順との差が走査順への依存になる。
    布あり逆順,
    /// 読込順のまま布だけを外す。読込順の布影領域を切り出す下敷きである。
    布なし読込順,
    /// 逆順のまま布だけを外す。逆順の布影領域を切り出す下敷きである。
    布なし逆順,
}

pub(super) struct 実行結果 {
    幅: usize,
    高さ: usize,
    rgba8: Vec<u8>,
}

impl 実行結果 {
    pub(super) fn 寸法(&self) -> (usize, usize) {
        (self.幅, self.高さ)
    }

    /// 画素を4バイトずつ先頭から並べた並び。バイト列の並べ方を知るのはこの型だけであり、判定側は画素の組で読む。
    pub(super) fn 画素列(&self) -> impl Iterator<Item = &[u8]> {
        self.rgba8.chunks_exact(4)
    }
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, シェーダー入口: &Path, 条件: 条件) -> Result<実行結果, String> {
    let ダンプ先 = 出力先.join(出力名);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--object-count", 描画対象数])
        .args(["--frames", フレーム数])
        .arg("--no-post")
        .args(条件別引数(条件))
        .arg("--shader-source")
        .arg(シェーダー入口)
        .arg("--dump-frame")
        .arg(&ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(&ダンプ先)?;
    Ok(実行結果 { 幅, 高さ, rgba8 })
}

fn 条件別引数(条件: 条件) -> Vec<&'static str> {
    match 条件 {
        条件::布あり読込順 => vec!["--cloth"],
        条件::布あり逆順 => vec!["--cloth", "--reverse-draw-order"],
        条件::布なし読込順 => Vec::new(),
        条件::布なし逆順 => vec!["--reverse-draw-order"],
    }
}
