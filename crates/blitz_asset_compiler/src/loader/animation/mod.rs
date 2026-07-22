//! アニメーションクリップの読込。CUBICSPLINEは判断42により拒否する。

mod channel_group;
mod keyframe;
mod output;
mod property;

use blitz_math::秒;

use blitz_engine::{アニメーションクリップ, ジョイントアニメーションチャンネル};

use crate::error::アセットコンパイルエラー;

use super::document::開いた文書;
use super::skin::関節解決;

pub(super) fn アニメーション一覧を取り出す(
    文書: &開いた文書,
    関節解決: &関節解決,
    ジョイント数: usize,
) -> Result<Vec<アニメーションクリップ>, アセットコンパイルエラー> {
    文書
        .document
        .animations()
        .enumerate()
        .map(|(添字, アニメーション)| {
            let 名前 = アニメーション.name().map(str::to_string).unwrap_or_else(|| format!("animation_{添字}"));
            let ジョイントチャンネル一覧 =
                channel_group::ジョイントチャンネル一覧を取り出す(文書, &アニメーション, 関節解決, ジョイント数)?;
            let 継続秒 = 継続秒を求める(&ジョイントチャンネル一覧);
            Ok(アニメーションクリップ {
                名前,
                継続秒,
                ジョイントチャンネル一覧,
            })
        })
        .collect()
}

/// 全チャンネルの最終キー時刻の最大値を継続秒とする(glTFに明示の尺フィールドが無いため)。
fn 継続秒を求める(一覧: &[ジョイントアニメーションチャンネル]) -> 秒 {
    let mut 最大 = 秒::default();
    for チャンネル in 一覧 {
        for 候補 in [
            チャンネル.平行移動.as_ref().and_then(|c| c.時刻列.last()),
            チャンネル.回転.as_ref().and_then(|c| c.時刻列.last()),
            チャンネル.スケール.as_ref().and_then(|c| c.時刻列.last()),
        ]
        .into_iter()
        .flatten()
        {
            if *候補 > 最大 {
                最大 = *候補;
            }
        }
    }
    最大
}
