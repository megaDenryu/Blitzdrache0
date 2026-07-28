//! 1つの任意キーフレームチャンネルを検証して読む。

use blitz_math::秒;

use super::super::super::super::アセット実行時形式エラー;
use super::super::super::bytes::読取位置;
use crate::asset::{interpolation_kind::補間種別, keyframe_channel::チャンネル};

const 時刻長: usize = 4;

pub(super) fn 読む<値>(
    入力: &mut 読取位置<'_>,
    値バイト長: usize,
    継続秒: f32,
    mut 値を読む: impl FnMut(&mut 読取位置<'_>) -> Result<値, アセット実行時形式エラー>,
) -> Result<Option<チャンネル<値>>, アセット実行時形式エラー> {
    match 入力.u8()? {
        0 => Ok(None),
        1 => {
            let 補間 = 補間を読む(入力.u8()?)?;
            let 最小要素長 = 時刻長.checked_add(値バイト長).ok_or(アセット実行時形式エラー::長さ表現不能)?;
            let 件数 = 入力.件数(最小要素長)?;
            if 件数 == 0 {
                return Err(アセット実行時形式エラー::キーフレームなし);
            }
            let 時刻列 = 時刻列を読む(入力, 件数, 継続秒)?;
            let mut 値列 = Vec::with_capacity(件数);
            for _ in 0..件数 {
                値列.push(値を読む(入力)?);
            }
            Ok(Some(チャンネル { 時刻列, 値列, 補間 }))
        }
        不正 => Err(アセット実行時形式エラー::不正な有無判別値(不正)),
    }
}

fn 補間を読む(種別: u8) -> Result<補間種別, アセット実行時形式エラー> {
    match 種別 {
        0 => Ok(補間種別::ステップ),
        1 => Ok(補間種別::線形),
        不正 => Err(アセット実行時形式エラー::未知の補間種別(不正)),
    }
}

fn 時刻列を読む(入力: &mut 読取位置<'_>, 件数: usize, 継続秒: f32) -> Result<Vec<秒>, アセット実行時形式エラー> {
    let mut 時刻列 = Vec::with_capacity(件数);
    let mut 前時刻 = None;
    for _ in 0..件数 {
        let 現在 = 入力.f32()?;
        if 現在 < 0.0 || 前時刻.is_some_and(|前| 現在 <= 前) {
            return Err(アセット実行時形式エラー::キーフレーム時刻順序違反);
        }
        if 現在 > 継続秒 {
            return Err(アセット実行時形式エラー::キーフレーム時刻範囲外);
        }
        時刻列.push(秒::生成する(現在));
        前時刻 = Some(現在);
    }
    Ok(時刻列)
}
