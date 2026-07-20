//! M1のDoD自動検証: shaders/triangle.slangを一時コピーへ複製し、
//! `--frames`付きでblitz_appを実行して終了コードで合否を報告する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断4」「判断10」。

use std::path::PathBuf;
use std::process::{Command, ExitCode};

const 検証フレーム数: &str = "480";

pub fn 実行する() -> ExitCode {
    let コピー先 = match シェーダーを一時コピーする() {
        Ok(パス) => パス,
        Err(誤り) => {
            eprintln!("[xtask] シェーダーの一時コピーに失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };

    println!(
        "[xtask] cargo run -p blitz_app -- --frames {検証フレーム数} --shader-source {} を実行",
        コピー先.display()
    );
    let 起動結果 = Command::new("cargo")
        .args([
            "run",
            "-p",
            "blitz_app",
            "--",
            "--frames",
            検証フレーム数,
            "--shader-source",
        ])
        .arg(&コピー先)
        .status();

    match 起動結果 {
        Ok(終了状態) if 終了状態.success() => {
            println!("[xtask] smoke成功: validation・ピクセル判定・ホットリロードすべて成功で終了した");
            ExitCode::SUCCESS
        }
        Ok(終了状態) => {
            eprintln!("[xtask] smoke失敗: blitz_appが終了コード{終了状態}で終了した");
            ExitCode::FAILURE
        }
        Err(起動誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {起動誤り}");
            ExitCode::FAILURE
        }
    }
}

/// リポジトリの`shaders/triangle.slang`を`target/smoke_shaders/`へ複製する。
/// スモークシナリオが監視対象ファイルを書き換えるため、リポジトリ本体は汚さない。
fn シェーダーを一時コピーする() -> Result<PathBuf, String> {
    let コピー先ディレクトリ = PathBuf::from("target/smoke_shaders");
    std::fs::create_dir_all(&コピー先ディレクトリ)
        .map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    let コピー先 = コピー先ディレクトリ.join("triangle.slang");
    std::fs::copy("shaders/triangle.slang", &コピー先)
        .map_err(|誤り| format!("shaders/triangle.slangのコピーに失敗した: {誤り}"))?;
    Ok(コピー先)
}
