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
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::シェーダーソース引数不正("--shader-sourceに値が指定されていない".to_string()))?;
    Ok(PathBuf::from(値))
}

pub(super) fn scene引数を処理する(引数: &mut Iter<String>) -> Result<String, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::シーン名引数不正("--sceneに値が指定されていない".to_string()))?;
    Ok(値.clone())
}

pub(super) fn asset_root引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::アセットルート引数不正("--asset-rootに値が指定されていない".to_string()))?;
    Ok(PathBuf::from(値))
}

pub(super) fn dump_frame引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::フレームダンプ引数不正("--dump-frameに値が指定されていない".to_string()))?;
    Ok(PathBuf::from(値))
}

pub(super) fn exposure引数を処理する(引数: &mut Iter<String>) -> Result<f32, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::露出引数不正("--exposureに値が指定されていない".to_string()))?;
    let 露出 = 値.parse::<f32>().map_err(|_| 起動エラー::露出引数不正(値.clone()))?;
    if !露出.is_finite() || 露出 <= 0.0 {
        return Err(起動エラー::露出引数不正(format!("正の有限値でない: {値}")));
    }
    Ok(露出)
}

pub(super) fn blend引数を処理する(引数: &mut Iter<String>) -> Result<f32, 起動エラー> {
    let 値 = 引数
        .next()
        .ok_or_else(|| 起動エラー::ブレンド引数不正("--blendに値が指定されていない".to_string()))?;
    let ブレンド = 値.parse::<f32>().map_err(|_| 起動エラー::ブレンド引数不正(値.clone()))?;
    if !(0.0..=1.0).contains(&ブレンド) {
        return Err(起動エラー::ブレンド引数不正(format!("0から1の範囲でない: {値}")));
    }
    Ok(ブレンド)
}
