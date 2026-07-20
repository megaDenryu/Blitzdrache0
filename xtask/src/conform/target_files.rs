//! 検査対象ファイルの走査。crates/・xtask/src/・shaders/ を再帰的に辿り、
//! 指定拡張子のファイル一覧を集める。

use std::path::{Path, PathBuf};

pub fn 対象ファイル一覧を集める(
    ルート一覧: &[&str],
    拡張子一覧: &[&str],
) -> Result<Vec<PathBuf>, String> {
    let mut 結果 = Vec::new();
    for ルート in ルート一覧 {
        再帰的に集める(Path::new(ルート), 拡張子一覧, &mut 結果)?;
    }
    結果.sort();
    Ok(結果)
}

fn 再帰的に集める(
    ディレクトリ: &Path,
    拡張子一覧: &[&str],
    結果: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if !ディレクトリ.exists() {
        return Ok(());
    }
    let 読み取り結果 = std::fs::read_dir(ディレクトリ)
        .map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", ディレクトリ.display()))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ = エントリ結果
            .map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", ディレクトリ.display()))?;
        let パス = エントリ.path();
        if パス.is_dir() {
            再帰的に集める(&パス, 拡張子一覧, 結果)?;
            continue;
        }
        let 拡張子が対象か = パス
            .extension()
            .and_then(|拡張子| 拡張子.to_str())
            .is_some_and(|拡張子| 拡張子一覧.contains(&拡張子));
        if 拡張子が対象か {
            結果.push(パス);
        }
    }
    Ok(())
}
