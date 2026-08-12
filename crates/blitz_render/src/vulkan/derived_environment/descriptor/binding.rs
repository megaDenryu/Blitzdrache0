//! 派生表現ディスクリプタのバインド番号と型の対応を持つ。レイアウト・プールの容量・セットへの書き込みが
//! すべてこの1箇所の対応を読むため、番号と型の食い違いが起こらない。
//!
//! 注意: 番号は`shaders/diffuse_irradiance.slang`と`shaders/specular_prefilter.slang`の`vk::binding`と一致させる。
//! 注意: 読む遠方環境も書き込み先も、レイアウトはすべてGENERALである。
//! レンダーグラフの画像用途「コンピュート読み」「コンピュート書き」が同じレイアウトへ遷移させており、
//! ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。
//!
//! 遠方環境をサンプラー付きでなく画像だけで束縛するのは、参照が補間を伴わない1テクセルの読み出しだからである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号, 結ぶ現物
};

const 宣言: 宣言した束縛の並び<2> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::SAMPLED_IMAGE, vk::ShaderStageFlags::COMPUTE),
    (束縛番号::生成する(1), vk::DescriptorType::STORAGE_IMAGE, vk::ShaderStageFlags::COMPUTE),
]);

pub(super) fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<2>, レンダラーエラー> {
    宣言.セットレイアウトを確保する(device)
}

pub(super) fn プールを作る(device: &ash::Device, セット数: u32) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = 宣言.プールの内訳(セット数);
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

pub(super) fn 書き込む(
    device: &ash::Device,
    セット: &宣言から割り当てたセット<2>,
    遠方環境の配列ビュー: vk::ImageView,
    書き込み先: vk::ImageView,
) {
    セット.書き込み先(device).並びの位置ごとに結ぶ([
        結ぶ現物::サンプラー無しの画像 {
            ビュー: 遠方環境の配列ビュー,
            レイアウト: vk::ImageLayout::GENERAL,
        },
        結ぶ現物::サンプラー無しの画像 {
            ビュー: 書き込み先,
            レイアウト: vk::ImageLayout::GENERAL,
        },
    ]);
}
