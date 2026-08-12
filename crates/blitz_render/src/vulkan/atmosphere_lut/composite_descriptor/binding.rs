//! 空中遠近合成ディスクリプタのバインド番号と型の対応を持つ。レイアウト・プールの容量・セットへの書き込みが
//! すべてこの1箇所の対応を読むため、番号と型の食い違いが起こらない。
//!
//! 注意: 番号は`shaders/aerial_composite.slang`の`vk::binding`のset1側と一致させる。
//! 注意: 深度のレイアウトはDEPTH_READ_ONLY_OPTIMAL、ボリュームのレイアウトはGENERALである。レンダーグラフの
//! 画像用途「深度シェーダー読み」と「焼いた画像の画素段参照」が同じレイアウトへ遷移させており、ここの値と
//! バリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。

use ash::vk;

use super::空中遠近合成の束縛先;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号, 結ぶ現物
};

/// 並びの位置。深度は毎フレーム結び直し、ボリュームは生成時に1度だけ結ぶ。
const 深度の位置: usize = 0;
const ボリュームの位置: usize = 1;

const 宣言: 宣言した束縛の並び<2> = 宣言した束縛の並び::生成する([
    (
        束縛番号::生成する(0),
        vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        vk::ShaderStageFlags::FRAGMENT,
    ),
    (
        束縛番号::生成する(1),
        vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        vk::ShaderStageFlags::FRAGMENT,
    ),
]);

pub(super) fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<2>, レンダラーエラー> {
    宣言.セットレイアウトを確保する(device)
}

pub(super) fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = descriptor_common::セット数();
    let プールサイズ一覧 = 宣言.プールの内訳(セット数);
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

/// ボリュームを結ぶ。生成時に1度だけ書けば足りるのは、ボリュームが起動時に1度確保して使い回す画像だからである。
pub(super) fn ボリュームを書き込む(
    device: &ash::Device,
    セット: &宣言から割り当てたセット<2>,
    sampler: vk::Sampler,
    束縛先: &空中遠近合成の束縛先,
) {
    書き込む(device, セット, ボリュームの位置, sampler, 束縛先.空中遠近ビュー, vk::ImageLayout::GENERAL);
}

/// 深度を結ぶ。毎フレーム呼ぶ理由は`composite_descriptor`の冒頭にある。
pub(super) fn 深度を書き込む(
    device: &ash::Device, セット: &宣言から割り当てたセット<2>, sampler: vk::Sampler, 深度ビュー: vk::ImageView
) {
    書き込む(device, セット, 深度の位置, sampler, 深度ビュー, vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL);
}

fn 書き込む(
    device: &ash::Device,
    セット: &宣言から割り当てたセット<2>,
    位置: usize,
    sampler: vk::Sampler,
    ビュー: vk::ImageView,
    レイアウト: vk::ImageLayout,
) {
    セット.書き込み先(device).並びの位置へ結ぶ(
        位置,
        結ぶ現物::サンプラー付きの画像 {
            ビュー,
            サンプラー: sampler,
            レイアウト,
        },
    );
}
