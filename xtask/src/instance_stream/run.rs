//! 地形と植生が同居する25チャンク世界を、本番のストリーミング経路と固定経路で1回走らせる工程。
//! 受け取るのは名前とフレーム数と追加引数、返すのは終了時報告の2種の読み取り結果と最終フレームの読み戻し画像である。
//! 監視対象シェーダーへリポジトリ本体でなく一時コピーを渡すのは、スモーク計画の書き換えがリポジトリのファイルを変えないようにするためである。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::report_parse::計数報告;
use crate::streaming_report::ストリーミング要約報告;

/// 検証用地形世界の実行時アセットの置き場。`compile-assets`の地形世界の既定出力ルートと同じ値である。
const アセットルート: &str = "target/terrain_assets";

/// 起動時シーン。レンダラーはチャンクが1つも常駐しない期間にも描画対象を要求するため、束ID0を占めるこの対象が要る。
const 起動時シーン: &str = "terrain_origin";

/// RAMとVRAMの上限。地形と植生が同居した25チャンクの実測に対して余裕を持つ値であり、この検収は縮退を見ない。
const 上限バイト数: &str = "16777216";

/// ライティングを止めるのは、この検収が植生の画素を地形の画素とベースカラーの色だけで見分けるためである。
/// ポスト処理も止めるのは、明るさの圧縮が色を移して同じ見分けを鈍らせるためである。
/// 空を止めるのは、地形世界が空を持つ方針であり、空が塗った画素を番兵背景と見分けられなくなるためである。
const 共通引数: [&str; 13] = [
    "--scene",
    起動時シーン,
    "--asset-root",
    アセットルート,
    "--streaming",
    "--streaming-preload-radius",
    "2",
    "--instance-stream-route",
    "--report-streaming-summary",
    "--report-memory",
    "--report-draw-issue",
    "--unlit",
    "--no-sky",
];

pub(super) struct 実行 {
    pub(super) 名前: String,
    pub(super) 計数: 計数報告,
    pub(super) 要約: ストリーミング要約報告,
    pub(super) 画像: 読み戻し画像,
}

pub(super) struct 読み戻し画像 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    pub(super) rgba8: Vec<u8>,
}

pub(super) fn 走らせる(
    出力先: &Path, 名前: &str, シェーダー入口: &Path, フレーム数: &str, 追加引数: &[&str]
) -> Result<実行, String> {
    let ダンプ先 = 出力先.join(名前);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--"])
        .args(共通引数)
        .args(["--no-post", "--frames", フレーム数])
        .args(["--streaming-ram-limit", 上限バイト数, "--streaming-vram-limit", 上限バイト数])
        .args(追加引数)
        .arg("--shader-source")
        .arg(シェーダー入口)
        .arg("--dump-frame")
        .arg(&ダンプ先);
    let 出力 = コマンド.output().map_err(|誤り| format!("blitz_appを起動できなかった({名前}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    print!("{標準出力}");
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({名前})", 出力.status));
    }
    Ok(実行 {
        名前: 名前.to_string(),
        計数: crate::report_parse::取り出す(&標準出力)?,
        要約: crate::streaming_report::取り出す(&標準出力)?,
        画像: 読み込む(&ダンプ先)?,
    })
}

fn 読み込む(ダンプ先: &Path) -> Result<読み戻し画像, String> {
    let 寸法パス = PathBuf::from(ダンプ先).with_extension("size");
    let 寸法 = std::fs::read_to_string(&寸法パス).map_err(|誤り| format!("{}を読めない: {誤り}", 寸法パス.display()))?;
    let 数一覧: Vec<usize> = 寸法.split_whitespace().filter_map(|語| 語.parse().ok()).collect();
    let [幅, 高さ] = 数一覧[..] else {
        return Err(format!("{}の寸法が「幅 高さ」の2数でない: {寸法}", 寸法パス.display()));
    };
    let rgba8 = std::fs::read(PathBuf::from(ダンプ先).with_extension("raw"))
        .map_err(|誤り| format!("{}の読み戻し画像を読めない: {誤り}", ダンプ先.display()))?;
    let 期待長 = 幅
        .checked_mul(高さ)
        .and_then(|数| 数.checked_mul(4))
        .ok_or_else(|| "読み戻し画像の寸法が大きすぎる".to_string())?;
    if rgba8.len() != 期待長 {
        return Err(format!("寸法とRGBA8長が違う: {期待長} と {}", rgba8.len()));
    }
    Ok(読み戻し画像 { 幅, 高さ, rgba8 })
}
