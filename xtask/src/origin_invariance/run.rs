//! 検査1条件ぶんのblitz_app起動と読み戻し画像の取り込み。
//! 担当する工程: 検査条件を1つ受け取り、最終フレームの読み戻しを`<ベース名>.raw`と`<ベース名>.size`から画素バイト列として返す。

use std::path::Path;
use std::process::Command;

/// 読み戻し画像の画素バイト列と寸法宣言。寸法まで含めて比べることで、条件ごとに解像度が変わった場合も不一致として検出できる。
pub(super) struct 読み戻し画像 {
    pub(super) 寸法宣言: String,
    pub(super) 画素バイト列: Vec<u8>,
}

/// 1条件ぶんの起動設定。大域ずらし量は軸ごとに違う値を与えられるよう3成分で持ち、カメラずれは負の対照でのみ0以外にする。
pub(super) struct 検査条件 {
    pub(super) 大域ずらし量: [f64; 3],
    pub(super) カメラずれメートル: f64,
}

pub(super) fn 読み戻しを取る(出力先: &Path, ベース名: &str, 条件: &検査条件) -> Option<読み戻し画像> {
    let ダンプ先 = 出力先.join(ベース名);
    let 引数一覧 = 引数列を作る(&ダンプ先, 条件);
    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    match Command::new("cargo").args(&引数一覧).status() {
        Ok(状態) if 状態.success() => {}
        Ok(状態) => {
            eprintln!("[xtask] blitz_appが終了コード{状態}で失敗した(条件{ベース名})");
            return None;
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗した: {誤り}");
            return None;
        }
    }
    読み込む(&ダンプ先)
}

fn 引数列を作る(ダンプ先: &Path, 条件: &検査条件) -> Vec<String> {
    let [x, y, z] = 条件.大域ずらし量;
    vec![
        "run".to_string(),
        "-p".to_string(),
        "blitz_app".to_string(),
        "--".to_string(),
        "--frames".to_string(),
        super::フレーム数.to_string(),
        "--scene".to_string(),
        super::シーン名.to_string(),
        "--dump-frame".to_string(),
        ダンプ先.display().to_string(),
        "--global-offset".to_string(),
        format!("{x}"),
        format!("{y}"),
        format!("{z}"),
        "--camera-nudge".to_string(),
        format!("{}", 条件.カメラずれメートル),
    ]
}

fn 読み込む(ダンプ先: &Path) -> Option<読み戻し画像> {
    let rawパス = ダンプ先.with_extension("raw");
    let sizeパス = ダンプ先.with_extension("size");
    let 画素バイト列 = match std::fs::read(&rawパス) {
        Ok(バイト列) => バイト列,
        Err(誤り) => {
            eprintln!("[xtask] 読み戻し画像を読めなかった({}): {誤り}", rawパス.display());
            return None;
        }
    };
    let 寸法宣言 = match std::fs::read_to_string(&sizeパス) {
        Ok(内容) => 内容,
        Err(誤り) => {
            eprintln!("[xtask] 読み戻し寸法を読めなかった({}): {誤り}", sizeパス.display());
            return None;
        }
    };
    Some(読み戻し画像 {
        寸法宣言, 画素バイト列
    })
}
