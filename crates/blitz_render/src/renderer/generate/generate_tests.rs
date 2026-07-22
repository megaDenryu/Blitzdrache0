use crate::error::レンダラーエラー;
use crate::frame_composition::{フレーム構成, フレーム段階};

#[test]
fn 粒子素材に粒子段階が無ければ拒否する() {
    let 構成結果 = フレーム構成::生成する(&[フレーム段階::影, フレーム段階::シーン]);
    let Ok(構成) = 構成結果 else {
        panic!("テスト用フレーム構成を生成できなかった");
    };

    let 結果 = super::構成と素材を検査する(&構成, false, false, true);
    assert!(matches!(結果, Err(レンダラーエラー::フレーム構成素材不一致(フレーム段階::粒子))));
}
