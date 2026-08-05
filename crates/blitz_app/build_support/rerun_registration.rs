//! Cargoへ「どのファイルが変わったらビルドスクリプトを走らせ直すか」を伝える工程。受け取るのは
//! シェーダーディレクトリ、返すのは登録の成否である。シェーダーのコンパイルを1つも知らない。
//!
//! コンパイルの並びと分けるのは、この登録が失敗しても生成物は正しく、逆に登録を落とすと生成物は正しいまま
//! 古いSPIR-Vが残るという、症状の出方が違う関心事だからである。

use std::path::Path;

/// shaders/ディレクトリ自体と、直下の全.slangファイルをrerun-if-changed対象にする。
/// ディレクトリ自体も登録することで、ファイルの追加・削除にも追従する
/// (Cargoはディレクトリのmtimeでもこのトリガーを検知できる)。
pub(super) fn 登録する(ディレクトリ: &Path) -> Result<(), String> {
    println!("cargo:rerun-if-changed={}", ディレクトリ.display());
    let 読み取り結果 = std::fs::read_dir(ディレクトリ).map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ = エントリ結果.map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
        let パス = エントリ.path();
        if パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("slang") {
            println!("cargo:rerun-if-changed={}", パス.display());
        }
    }
    Ok(())
}
