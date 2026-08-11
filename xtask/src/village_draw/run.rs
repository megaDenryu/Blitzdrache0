//! 集落1回ぶんのblitz_app起動と、終了時報告の取り出し。受け取るのはダンプ先、返すのは標準出力である。
//!
//! シーン名を`prop_village`にすることが、集落を見下ろす初期カメラと、書き換えもピクセル判定も持たない検収計画を選ばせる。
//! 空は`--sky`で明示する。世界の方針が空ありになるのは名前が`terrain`で始まる世界だけであり、この世界はそこへ入らないためである。
//! 監視対象シェーダーは指定しない。指定しないと埋め込みシェーダーで走り、リポジトリの`shaders/`へ監視の入口が向かない。

use std::path::Path;
use std::process::Command;

use crate::report_heading::報告の見出し;

const アセットルート: &str = "target/village_assets";
const シーン名: &str = "prop_village";
const フレーム数: &str = "180";
const 条件名: &str = "village";

/// 一日内時刻17時。太陽が低く、小物の影が地面を長く横切るため、起伏と物の高さが影の形で読み取れる。
const 一日内秒: &str = "61200";

/// 終了時報告のうち、この入口が読ませたい部分の見出し。行の先頭から一致を見る。
const 報告の見出し一覧: [報告の見出し; 5] = [
    報告の見出し::生成する("描画発行内訳"),
    報告の見出し::生成する("CPU側フレーム間隔"),
    報告の見出し::生成する("パス別GPU時間"),
    報告の見出し::生成する("Vulkanメモリ確保"),
    crate::validation_count::検証層の指摘件数の見出し,
];

pub(super) fn 描画する(ダンプ先: &Path) -> Result<String, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .args([
            "--sky",
            "--report-gpu-times",
            "--report-frame-times",
            "--report-draw-issue",
            "--report-memory",
        ])
        .arg("--dump-frame")
        .arg(ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった: {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        return Err(format!("blitz_appが{}で失敗した", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 条件名)?;
    Ok(標準出力)
}

/// 報告の見出しとその内訳の行を並べ直して出す。内訳は見出しより深く字下げされているため、
/// 見出しに続く字下げ行を同じ塊として拾えば、報告ごとの読み方を持たずに全体を写せる。
pub(super) fn 報告を書き出す(標準出力: &str) {
    let mut 報告の内側 = false;
    for 行 in 標準出力.lines() {
        if 報告の見出し一覧.iter().any(|見出し| 見出し.行の始まりか(行)) {
            報告の内側 = true;
        } else if !行.starts_with(' ') {
            報告の内側 = false;
        }
        if 報告の内側 {
            println!("[village-draw] {行}");
        }
    }
}
