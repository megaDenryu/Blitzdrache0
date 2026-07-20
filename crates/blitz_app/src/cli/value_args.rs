//! 値を伴うCLI引数(`--frames` `--shader-source` `--scene` `--asset-root`)の解析。
//! `cli.rs`の行数分割のためだけに切り出した内部ヘルパー。

use std::path::PathBuf;
use std::slice::Iter;

use super::起動モード;
use crate::error::起動エラー;

pub(super) fn frames引数を処理する(引数: &mut Iter<String>) -> Result<起動モード, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::フレーム数引数不正("--framesに値が指定されていない".to_string()))?;
    let フレーム数 = 値.parse::<u32>().map_err(|_| 起動エラー::フレーム数引数不正(値.clone()))?;
    Ok(起動モード::スモーク実行 { フレーム数 })
}

pub(super) fn shader_source引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動エラー> {
    let 値 = 引数.next().ok_or_else(|| {
        起動エラー::シェーダーソース引数不正("--shader-sourceに値が指定されていない".to_string())
    })?;
    Ok(PathBuf::from(値))
}

pub(super) fn scene引数を処理する(引数: &mut Iter<String>) -> Result<String, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::シーン名引数不正("--sceneに値が指定されていない".to_string()))?;
    Ok(値.clone())
}

pub(super) fn asset_root引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動エラー> {
    let 値 = 引数.next().ok_or_else(|| {
        起動エラー::アセットルート引数不正("--asset-rootに値が指定されていない".to_string())
    })?;
    Ok(PathBuf::from(値))
}

pub(super) fn dump_frame引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動エラー> {
    let 値 = 引数.next().ok_or_else(|| {
        起動エラー::フレームダンプ引数不正("--dump-frameに値が指定されていない".to_string())
    })?;
    Ok(PathBuf::from(値))
}
