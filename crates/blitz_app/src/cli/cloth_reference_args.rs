//! 布のXPBDの参照比較を選ぶ引数(`--cloth-xpbd-reference` `--cloth-xpbd-reference-below-floor` `--cloth-xpbd-reference-bending` `--cloth-xpbd-reference-shape`)の解析。
//! 方式を選ぶ2つは布モードを参照比較にし、曲げのコンプライアンスと題材の形の2つは既に参照比較になっている布モードへ値を与える。
//! 後者を方式より前に置いた指定は、どの題材へ与えるかが無いため型付きの失敗にする(黙って読み捨てない)。

use std::slice::Iter;

use super::value_args::次の値を読む;
use super::{
    参照比較の床の下の固定点, 参照比較の題材の形, 布のコンプライアンス指定, 布の曲げのコンプライアンス指定, 布モード, 起動引数エラー
};

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
        曲げのコンプライアンス: 布の曲げのコンプライアンス指定::既定(),
        題材の形: 参照比較の題材の形::既定(),
    })
}

/// `--cloth-xpbd-reference-bending <曲げのコンプライアンス>`。既に参照比較の方式になっている布モードへ曲げのコンプライアンスを与える。
/// `--cloth-xpbd-reference`より前に置かれた指定は、どの題材へ与えるかが無いため型付きの失敗にする(黙って読み捨てない)。
pub(super) fn cloth_xpbd_reference_bending引数を処理する(
    引数: &mut Iter<String>,
    布モード: 布モード,
) -> Result<布モード, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--cloth-xpbd-reference-bending", 起動引数エラー::布の曲げのコンプライアンス不正)?;
    let 曲げ = 布の曲げのコンプライアンス指定::綴りから解析する(値)?;
    match 布モード {
        布モード::XPBD参照比較 {
            コンプライアンス,
            床の下の固定点,
            題材の形,
            ..
        } => Ok(布モード::XPBD参照比較 {
            コンプライアンス,
            床の下の固定点,
            曲げのコンプライアンス: 曲げ,
            題材の形,
        }),
        _ => Err(起動引数エラー::布の曲げのコンプライアンス不正(
            "--cloth-xpbd-referenceの後に指定する".to_string(),
        )),
    }
}

/// `--cloth-xpbd-reference-shape <綴り>`。既に参照比較の方式になっている布モードへ題材の形(敷き方と固定)を与える。
pub(super) fn cloth_xpbd_reference_shape引数を処理する(
    引数: &mut Iter<String>,
    布モード: 布モード,
) -> Result<布モード, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--cloth-xpbd-reference-shape", 起動引数エラー::参照比較の題材の形不正)?;
    let 形 = 参照比較の題材の形::綴りから解析する(値)?;
    match 布モード {
        布モード::XPBD参照比較 {
            コンプライアンス,
            床の下の固定点,
            曲げのコンプライアンス,
            ..
        } => Ok(布モード::XPBD参照比較 {
            コンプライアンス,
            床の下の固定点,
            曲げのコンプライアンス,
            題材の形: 形,
        }),
        _ => Err(起動引数エラー::参照比較の題材の形不正(
            "--cloth-xpbd-referenceの後に指定する".to_string(),
        )),
    }
}
