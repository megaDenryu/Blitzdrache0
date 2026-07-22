//! 1チャンネルの読み取り結果をプロパティ種別ごとにジョイントチャンネルへ格納する。
//! モーフターゲットウェイトはM8スコープ外のため読み飛ばす(判断42)。

use blitz_engine::{ジョイントアニメーションチャンネル, チャンネル, 補間種別};

use crate::error::アセットコンパイルエラー;

use super::keyframe;
use super::output::{スケール出力を読む, 回転出力を読む, 平行移動出力を読む};

pub(super) fn プロパティを反映する<'a, 's, F>(
    読み取り器: gltf::animation::util::Reader<'a, 's, F>,
    プロパティ: gltf::animation::Property,
    補間元: gltf::animation::Interpolation,
    格納先: &mut ジョイントアニメーションチャンネル,
) -> Result<(), アセットコンパイルエラー>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    if matches!(プロパティ, gltf::animation::Property::MorphTargetWeights) {
        return Ok(());
    }
    let 補間 = keyframe::補間種別を変換する(補間元)?;
    let 時刻列 = 時刻列を読む(&読み取り器)?;

    match プロパティ {
        gltf::animation::Property::Translation => {
            格納先.平行移動 = Some(組み立てる(時刻列, 平行移動出力を読む(&読み取り器)?, 補間));
        }
        gltf::animation::Property::Rotation => {
            格納先.回転 = Some(組み立てる(時刻列, 回転出力を読む(&読み取り器)?, 補間));
        }
        gltf::animation::Property::Scale => {
            格納先.スケール = Some(組み立てる(時刻列, スケール出力を読む(&読み取り器)?, 補間));
        }
        gltf::animation::Property::MorphTargetWeights => {}
    }
    Ok(())
}

fn 組み立てる<値>(時刻列: Vec<f32>, 値列: Vec<値>, 補間: 補間種別) -> チャンネル<値> {
    チャンネル {
        時刻列: keyframe::時刻列を変換する(時刻列),
        値列,
        補間,
    }
}

fn 時刻列を読む<'a, 's, F>(読み取り器: &gltf::animation::util::Reader<'a, 's, F>) -> Result<Vec<f32>, アセットコンパイルエラー>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    読み取り器
        .read_inputs()
        .map(Iterator::collect)
        .ok_or_else(|| アセットコンパイルエラー::解析失敗("アニメーション入力の読み取りに失敗した".to_string()))
}
