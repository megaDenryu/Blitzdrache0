//! 値を伴うCLI引数(`--frames` `--benchmark-frames` `--shader-source`等)の解析。
//! 各関数は引数イテレータから次の1語を取り出し、値の欠落・解析失敗・範囲外をそれぞれ型付きエラーにする。

use std::path::PathBuf;
use std::slice::Iter;

use super::{描画対象数, 起動モード, 起動引数エラー};

/// 値の欠落は引数ごとに違う型付きエラーになるため、エラーの作り方を引数で受け取る。
pub(super) fn 次の値を読む<'引数>(
    引数: &mut Iter<'引数, String>,
    引数名: &str,
    欠落エラー: fn(String) -> 起動引数エラー,
) -> Result<&'引数 String, 起動引数エラー> {
    引数.next().ok_or_else(|| 欠落エラー(format!("{引数名}に値が指定されていない")))
}

pub(super) fn frames引数を処理する(引数: &mut Iter<String>) -> Result<起動モード, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--frames", 起動引数エラー::フレーム数不正)?;
    let フレーム数 = 値.parse::<u32>().map_err(|_| 起動引数エラー::フレーム数不正(値.clone()))?;
    Ok(起動モード::スモーク実行 { フレーム数 })
}

pub(super) fn benchmark_frames引数を処理する(引数: &mut Iter<String>) -> Result<起動モード, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--benchmark-frames", 起動引数エラー::フレーム数不正)?;
    let フレーム数 = 値.parse::<u32>().map_err(|_| 起動引数エラー::フレーム数不正(値.clone()))?;
    Ok(起動モード::ベンチ実行 { フレーム数 })
}

pub(super) fn shader_source引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--shader-source", 起動引数エラー::シェーダーソース不正)?;
    Ok(PathBuf::from(値))
}

pub(super) fn scene引数を処理する(引数: &mut Iter<String>) -> Result<String, 起動引数エラー> {
    Ok(次の値を読む(引数, "--scene", 起動引数エラー::シーン名不正)?.clone())
}

pub(super) fn asset_root引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--asset-root", 起動引数エラー::アセットルート不正)?;
    Ok(PathBuf::from(値))
}

pub(super) fn object_count引数を処理する(引数: &mut Iter<String>) -> Result<描画対象数, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--object-count", 起動引数エラー::描画対象数不正)?;
    let 数 = 値.parse::<u32>().map_err(|_| 起動引数エラー::描画対象数不正(値.clone()))?;
    描画対象数::生成する(数).map_err(起動引数エラー::描画対象数不正)
}

pub(super) fn dump_frame引数を処理する(引数: &mut Iter<String>) -> Result<PathBuf, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--dump-frame", 起動引数エラー::フレームダンプ不正)?;
    Ok(PathBuf::from(値))
}

/// `--streaming-ram-limit` `--streaming-vram-limit`のバイト数を読む。予算は1バイト以上でなければ生成できないため0を拒否する。
pub(super) fn ストリーミング上限引数を処理する(引数: &mut Iter<String>, 引数名: &str) -> Result<u64, 起動引数エラー> {
    let 値 = 次の値を読む(引数, 引数名, 起動引数エラー::ストリーミング上限不正)?;
    let バイト数 = 値
        .parse::<u64>()
        .map_err(|_| 起動引数エラー::ストリーミング上限不正(format!("{引数名}: {値}")))?;
    if バイト数 == 0 {
        return Err(起動引数エラー::ストリーミング上限不正(
            format!("{引数名}は1以上でなければならない"),
        ));
    }
    Ok(バイト数)
}

pub(super) fn exposure引数を処理する(引数: &mut Iter<String>) -> Result<f32, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--exposure", 起動引数エラー::露出不正)?;
    let 露出 = 値.parse::<f32>().map_err(|_| 起動引数エラー::露出不正(値.clone()))?;
    if !露出.is_finite() || 露出 <= 0.0 {
        return Err(起動引数エラー::露出不正(format!("正の有限値でない: {値}")));
    }
    Ok(露出)
}

pub(super) fn blend引数を処理する(引数: &mut Iter<String>) -> Result<f32, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--blend", 起動引数エラー::ブレンド不正)?;
    let ブレンド = 値.parse::<f32>().map_err(|_| 起動引数エラー::ブレンド不正(値.clone()))?;
    if !(0.0..=1.0).contains(&ブレンド) {
        return Err(起動引数エラー::ブレンド不正(format!("0から1の範囲でない: {値}")));
    }
    Ok(ブレンド)
}
