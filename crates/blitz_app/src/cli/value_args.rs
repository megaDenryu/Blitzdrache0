//! 値を伴うCLI引数(`--frames` `--benchmark-frames` `--shader-source`等)の解析。
//! 各関数は引数イテレータから次の1語を取り出し、値の欠落・解析失敗・範囲外をそれぞれ型付きエラーにする。

use std::path::PathBuf;
use std::slice::Iter;

use super::{
    フレームダンプ指定, 参照比較の床の下の固定点, 布のコンプライアンス指定, 布モード, 描画対象数, 起動モード, 起動引数エラー
};

/// 値の欠落は引数ごとに違う型付きエラーになるため、エラーの作り方を引数で受け取る。
pub(in crate::cli) fn 次の値を読む<'引数>(
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

/// `--cloth-xpbd-reference <コンプライアンス>`と`--cloth-xpbd-reference-below-floor <コンプライアンス>`。布をXPBDの参照比較の方式で起こし、
/// 構造とせん断へ同じコンプライアンスを与える。後者は目標が床の下にある世界固定点を1本持つ題材を選ぶ。
pub(super) fn cloth_xpbd_reference引数を処理する(
    引数: &mut Iter<String>,
    引数名: &str,
    床の下の固定点: 参照比較の床の下の固定点,
) -> Result<布モード, 起動引数エラー> {
    let 値 = 次の値を読む(引数, 引数名, 起動引数エラー::布のコンプライアンス不正)?;
    Ok(布モード::XPBD参照比較 {
        コンプライアンス: 布のコンプライアンス指定::綴りから解析する(値)?,
        床の下の固定点,
    })
}

pub(super) fn scene引数を処理する(引数: &mut Iter<String>) -> Result<super::起動時シーン, 起動引数エラー> {
    super::起動時シーン::綴りから解析する(次の値を読む(引数, "--scene", 起動引数エラー::シーン名不正)?)
}

pub(super) fn object_count引数を処理する(引数: &mut Iter<String>) -> Result<描画対象数, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--object-count", 起動引数エラー::描画対象数不正)?;
    let 数 = 値.parse::<u32>().map_err(|_| 起動引数エラー::描画対象数不正(値.clone()))?;
    描画対象数::生成する(数).map_err(|誤り| 起動引数エラー::描画対象数不正(誤り.to_string()))
}

/// `--dump-frame`と`--dump-hdr-frame`と`--dump-depth-frame`は値の読み方が同じであるため、どれもこの1つが受ける。
/// どの画像を選ぶかと、既に選ばれていたときに失敗させるかは指定の型が決める。
pub(super) fn フレームダンプ引数を反映する(
    指定: &mut フレームダンプ指定,
    引数: &mut Iter<String>,
    引数名: &str,
) -> Result<(), 起動引数エラー> {
    let 基準名 = PathBuf::from(次の値を読む(引数, 引数名, 起動引数エラー::フレームダンプ不正)?);
    指定.引数から設定する(引数名, 基準名)
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

pub(super) fn 先読み半径引数を処理する(引数: &mut Iter<String>) -> Result<u8, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--streaming-preload-radius", 起動引数エラー::先読み半径不正)?;
    let 半径 = 値.parse::<u8>().map_err(|_| 起動引数エラー::先読み半径不正(値.clone()))?;
    if 半径 > 16 {
        return Err(起動引数エラー::先読み半径不正(format!("上限16を超える: {半径}")));
    }
    Ok(半径)
}

pub(super) fn exposure引数を処理する(引数: &mut Iter<String>) -> Result<super::露出倍率, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--exposure", 起動引数エラー::露出不正)?;
    let 実数 = 値.parse::<f32>().map_err(|_| 起動引数エラー::露出不正(値.clone()))?;
    super::露出倍率::生成する(実数).map_err(|誤り| 起動引数エラー::露出不正(誤り.to_string()))
}

pub(super) fn blend引数を処理する(
    引数: &mut Iter<String>,
) -> Result<super::アニメーションのブレンド係数, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--blend", 起動引数エラー::ブレンド不正)?;
    let 実数 = 値.parse::<f32>().map_err(|_| 起動引数エラー::ブレンド不正(値.clone()))?;
    super::アニメーションのブレンド係数::生成する(実数).map_err(|誤り| 起動引数エラー::ブレンド不正(誤り.to_string()))
}
