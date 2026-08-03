//! 差し替えを挟む1回の起動と、そこから出る2枚の読み戻し画像の取り込み。受け取るのは出力先と監視先のエントリファイル、
//! 返すのは差し替え前と差し替え後の実行結果である。
//!
//! 起動が1回なのがこの検収の要点である。アプリが計画にしたがって差し替え前の絵を`_before`の名前で書き出し、
//! 監視先のシェーダーを書き換え、最終フレームで差し替え後の絵を書き出す。2回起動すると、同じプロセスの中で
//! パイプラインが入れ替わったことを見たことにならない。
//!
//! 昼を選ぶのは、大気から焼いた遠方環境が最も明るく、間接光が板の画素へ強く出る時刻だからである。間接光が0に近い
//! 時刻では、誤って定数近似の画素段が載っても絵が動かず、画像のバイト一致が判定の意味を失う。
//! ポスト処理を通したままにするのは、明るさの圧縮が無いと昼の板が飽和して同じ理由で差が消えるためである。

use std::path::Path;
use std::process::Command;

use crate::vegetation_run::実行結果;

const アセットルート: &str = "target/runtime_assets";
/// 0.5秒ごとの確認・3本の再コンパイル・全シーンキーのパイプライン再構築を書き換えから最終フレームまでに収めるため、
/// 既存の読み戻し検収より長く回す。
const フレーム数: &str = "300";
const シーン名: &str = "indirect_probe";
/// 昼の一日内秒。太陽が高く、大気から焼いた遠方環境が最も明るくなる。
const 昼の一日内秒: &str = "43200";

pub(super) fn 差し替えを挟んで描画する(出力先: &Path, 監視先: &Path) -> Result<(実行結果, 実行結果), String> {
    let ダンプ先 = 出力先.join(シーン名);
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 昼の一日内秒])
        .args(["--no-sky", "--shader-reload", "--report-memory"])
        .arg("--shader-source")
        .arg(監視先)
        .arg("--dump-frame")
        .arg(&ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった: {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した", 出力.status));
    }
    let 差し替え前 = 読み込む(&出力先.join(format!("{シーン名}_before")), &標準出力)?;
    let 差し替え後 = 読み込む(&ダンプ先, &標準出力)?;
    Ok((差し替え前, 差し替え後))
}

fn 読み込む(ダンプ先: &Path, 標準出力: &str) -> Result<実行結果, String> {
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(ダンプ先)?;
    Ok(実行結果 {
        標準出力: 標準出力.to_string(),
        幅,
        高さ,
        rgba8,
    })
}
