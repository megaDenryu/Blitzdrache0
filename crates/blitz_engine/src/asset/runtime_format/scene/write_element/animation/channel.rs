//! 1つの任意キーフレームチャンネルを書く。

use super::super::super::super::アセット実行時形式エラー;
use super::super::super::bytes::書込先;
use crate::asset::{interpolation_kind::補間種別, keyframe_channel::チャンネル};

pub(super) fn 書く<const 成分数: usize>(
    出力: &mut 書込先,
    チャンネル: Option<&チャンネル<[f32; 成分数]>>,
    継続秒: f32,
) -> Result<(), アセット実行時形式エラー> {
    let Some(チャンネル) = チャンネル else {
        出力.u8(0);
        return Ok(());
    };
    検査する(チャンネル, 継続秒)?;
    出力.u8(1);
    出力.u8(match チャンネル.補間 {
        補間種別::ステップ => 0,
        補間種別::線形 => 1,
    });
    出力.件数(チャンネル.時刻列.len())?;
    for 時刻 in &チャンネル.時刻列 {
        出力.f32(時刻.値())?;
    }
    for 値 in &チャンネル.値列 {
        for &成分 in 値 {
            出力.f32(成分)?;
        }
    }
    Ok(())
}

fn 検査する<const 成分数: usize>(
    チャンネル: &チャンネル<[f32; 成分数]>, 継続秒: f32
) -> Result<(), アセット実行時形式エラー> {
    if チャンネル.時刻列.is_empty() {
        return Err(アセット実行時形式エラー::キーフレームなし);
    }
    if チャンネル.時刻列.len() != チャンネル.値列.len() {
        return Err(アセット実行時形式エラー::キーフレーム数不一致 {
            時刻数: チャンネル.時刻列.len(),
            値数: チャンネル.値列.len(),
        });
    }
    let mut 前時刻 = None;
    for 時刻 in &チャンネル.時刻列 {
        let 現在 = 時刻.値();
        if !現在.is_finite() || 現在 < 0.0 || 前時刻.is_some_and(|前| 現在 <= 前) {
            return Err(アセット実行時形式エラー::キーフレーム時刻順序違反);
        }
        if 現在 > 継続秒 {
            return Err(アセット実行時形式エラー::キーフレーム時刻範囲外);
        }
        前時刻 = Some(現在);
    }
    Ok(())
}
