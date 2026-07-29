//! 読み取った係数の並びをリトルエンディアンのf32列としてファイルへ書く工程。
//! 受け取るのは出力パスと値の並び、返すのは成否である。読み手(`blitz_engine`の空データセット)は
//! `include_bytes!`で取り込んで4バイトずつ`f32::from_le_bytes`で戻す。

use std::path::Path;

pub(super) fn 書き出す(出力パス: &Path, 値一覧: &[f32]) -> Result<(), String> {
    if let Some(親) = 出力パス.parent() {
        std::fs::create_dir_all(親).map_err(|誤り| format!("{}を作れない: {誤り}", 親.display()))?;
    }
    let mut バイト列 = Vec::with_capacity(値一覧.len() * 4);
    for 値 in 値一覧 {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    std::fs::write(出力パス, &バイト列).map_err(|誤り| format!("{}へ書けない: {誤り}", 出力パス.display()))
}
