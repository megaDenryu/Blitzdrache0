//! 1回ぶんのblitz_app起動。受け取るのは実行の呼び名、返すのはその実行の標準出力である。
//!
//! 目視見本の庭を使うのは、この世界が時間再構成方式に履歴混合を宣言しており、パスが実際に積まれる世界そのもの
//! だからである。空も自動露出もポスト処理も本番の経路のまま残すのは、再構成が読む色が本番の絵と同じ
//! HDR中間画像の中身だからである。
//!
//! フレーム数を目視見本と同じ本数に採るのは、空と間接照明の焼き上げが定常へ入った状態で観測するためである。
//! 焼き上げが進んでいる間はフレーム間差分が再構成と無関係な要因で動く。

use std::process::Command;

/// 描くフレーム数。`terrain-visual`と同じ本数である。
const フレーム数: &str = "120";
/// 正午。時刻を1つに固定して、読み手が条件を推測しなくてよいようにする。
const 一日内秒: &str = "43200";
/// 合成入力の突き合わせの本数。履歴を有効にする1枚と突き合わせる1枚が最終フレームで走るため、そこまで進めば足りる。
const 注入のフレーム数: &str = "12";

pub(super) fn 観測を採る(呼び名: &str) -> Result<String, String> {
    走らせる(呼び名, フレーム数, "--report-temporal-reconstruction")
}

/// 合成入力の突き合わせの1回。焼き上げの進み具合に依存しないため短い本数で足りる。4枚とも検収が決めた値へ
/// 差し替わるため、空も間接照明も結果へ1画素も現れない。
pub(super) fn 合成入力の観測を採る() -> Result<String, String> {
    走らせる("合成入力の突き合わせ", 注入のフレーム数, "--report-temporal-reconstruction-injection")
}

fn 走らせる(呼び名: &str, 描くフレーム数: &str, 報告の指定: &str) -> Result<String, String> {
    println!("[xtask] temporal-reconstruction実行: {呼び名}");
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", crate::visual_sample_world::シーン名])
        .args(["--asset-root", crate::visual_sample_world::アセットルート])
        .args(["--frames", 描くフレーム数])
        .args(["--time-of-day", 一日内秒])
        .arg(報告の指定)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({呼び名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({呼び名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 呼び名)?;
    Ok(標準出力)
}
