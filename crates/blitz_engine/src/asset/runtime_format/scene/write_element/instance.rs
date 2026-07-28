//! インスタンス群の配置列と境界を書く。読み取り側と同じ並び(平行移動3・回転4・スケール3、包囲領域6、境界球4)で出す。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use crate::asset::instance::{個体配置, 群境界};

pub(in crate::asset::runtime_format::scene) fn 配置列を書く(
    出力: &mut 書込先,
    配置一覧: &[個体配置],
) -> Result<(), アセット実行時形式エラー> {
    出力.件数(配置一覧.len())?;
    for 配置 in 配置一覧 {
        for 値 in 配置.平行移動().into_iter().chain(配置.回転()).chain(配置.スケール()) {
            出力.f32(値)?;
        }
    }
    Ok(())
}

pub(in crate::asset::runtime_format::scene) fn 境界を書く(
    出力: &mut 書込先, 境界: 群境界
) -> Result<(), アセット実行時形式エラー> {
    let 包囲領域 = 境界.群の包囲領域();
    for 値 in 包囲領域.最小().into_iter().chain(包囲領域.最大()) {
        出力.f32(値)?;
    }
    let 球 = 境界.原型の境界球();
    for 値 in 球.中心() {
        出力.f32(値)?;
    }
    出力.f32(球.半径())
}
