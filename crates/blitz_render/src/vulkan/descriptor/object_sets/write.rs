//! 割り当て済みのディスクリプタセットへ、そのセットが束ねる資源を書き込む工程。束の読込時に1度だけ呼ばれる。
//! 走査順は割り当ての並び(描画対象の順、その中で材質スロットの順、その中でフレームスロットの順)と同じであり、
//! 位置の導出は`slot_index`が持つ配置に一任する。

use ash::vk;

use super::slot_index::スロット別セット配置;
use super::描画対象ディスクリプタ参照;
use crate::vulkan::descriptor::{buffer_binding, set, shadow_binding};
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::uniform::フレームシェーダー定数一式;

pub(super) fn 全セットの内容を書き込む(
    device: &ash::Device,
    set一覧: &[vk::DescriptorSet],
    配置: &スロット別セット配置,
    対象別参照一覧: &[Vec<描画対象ディスクリプタ参照<'_>>],
    シェーダー定数: &フレームシェーダー定数一式,
    シャドウマップ: &シャドウマップ,
) {
    for (描画対象添字, スロット別参照) in 対象別参照一覧.iter().enumerate() {
        for (スロット添字, 参照) in スロット別参照.iter().enumerate() {
            for フレーム添字 in フレームスロット添字::全スロット() {
                let 位置 = match 配置.位置(描画対象添字, スロット添字, フレーム添字) {
                    Some(値) => 値,
                    None => panic!("書き込む対象の添字がディスクリプタセットの配置の外だった"),
                };
                let Some(set) = set一覧.get(位置).copied() else {
                    panic!("ディスクリプタセット一覧が配置の示す位置を持たない");
                };
                セット1つへ書き込む(device, set, 参照, シェーダー定数, シャドウマップ, フレーム添字);
            }
        }
    }
}

fn セット1つへ書き込む(
    device: &ash::Device,
    set: vk::DescriptorSet,
    参照: &描画対象ディスクリプタ参照<'_>,
    シェーダー定数: &フレームシェーダー定数一式,
    シャドウマップ: &シャドウマップ,
    フレーム添字: フレームスロット添字,
) {
    set::テクスチャバインディングを書き込む(device, set, 参照.テクスチャ);
    buffer_binding::ビューとシーンパスの定数を書き込む(device, set, シェーダー定数.ビューとシーンパスのbuffer(フレーム添字));
    buffer_binding::多段影の定数を書き込む(device, set, シェーダー定数.多段影のbuffer(フレーム添字));
    buffer_binding::空パスの定数を書き込む(device, set, シェーダー定数.空パスのbuffer(フレーム添字));
    buffer_binding::描画対象シェーダー定数を書き込む(device, set, 参照.シェーダー定数.buffer);
    buffer_binding::個体変換を書き込む(device, set, 参照.個体変換.0, 参照.個体変換.1);
    buffer_binding::可視id列を書き込む(device, set, 参照.可視id列.buffer(フレーム添字), 参照.可視id列.範囲());
    shadow_binding::シャドウマップバインディングを書き込む(device, set, シャドウマップ);
}
