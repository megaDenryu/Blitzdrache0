//! 多重散乱ディスクリプタのバインド番号と型の対応を持つ。レイアウト・プールの容量・セットへの書き込みが
//! すべてこの1箇所の対応を読むため、番号と型の食い違いが起こらない。
//!
//! 注意: 番号は`shaders/atmosphere_multiscatter.slang`の`vk::binding`と一致させる。
//! 注意: 読む側の透過率のベイク済み画像も書き込み先の多重散乱のベイク済み画像もレイアウトはGENERALである。
//! レンダーグラフの画像用途「コンピュート読み」「コンピュート書き」が同じレイアウトへ遷移させており、
//! ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。

use ash::vk;

use super::多重散乱の束縛先;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号, 結ぶ現物
};
use crate::vulkan::sync::フレームスロット添字;

const 宣言: 宣言した束縛の並び<3> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::UNIFORM_BUFFER, vk::ShaderStageFlags::COMPUTE),
    (
        束縛番号::生成する(1),
        vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        vk::ShaderStageFlags::COMPUTE,
    ),
    (束縛番号::生成する(2), vk::DescriptorType::STORAGE_IMAGE, vk::ShaderStageFlags::COMPUTE),
]);

pub(super) fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<3>, レンダラーエラー> {
    宣言.セットレイアウトを確保する(device)
}

pub(super) fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = descriptor_common::セット数();
    let プールサイズ一覧 = 宣言.プールの内訳(セット数);
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

pub(super) fn 書き込む(
    device: &ash::Device,
    セット: &宣言から割り当てたセット<3>,
    sampler: vk::Sampler,
    束縛先: &多重散乱の束縛先<'_>,
    添字: フレームスロット添字,
) {
    セット.書き込み先(device).並びの位置ごとに結ぶ([
        結ぶ現物::バッファ全体(束縛先.シェーダー定数一覧[添字.配列添字()]),
        結ぶ現物::サンプラー付きの画像 {
            ビュー: 束縛先.透過率ビュー,
            サンプラー: sampler,
            レイアウト: vk::ImageLayout::GENERAL,
        },
        結ぶ現物::サンプラー無しの画像 {
            ビュー: 束縛先.多重散乱ビュー,
            レイアウト: vk::ImageLayout::GENERAL,
        },
    ]);
}
