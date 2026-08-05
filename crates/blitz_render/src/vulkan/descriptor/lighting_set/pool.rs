//! 照明問い合わせのセットを取り出すディスクリプタプールの確保と、その件数の決め方。
//! 触れるのはプール1つだけであり、セットへ何を結ぶかは知らない。
//!
//! 件数を進行中フレームスロットの数から導くのは、スロットごとに1つのセットを持つ設計だからである。
//! 件数がスロット数へ追従しないと、スロットを増やしたときに割り当てが実行時に失敗する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;

#[cfg(test)]
mod count_tests;

/// 1つのセットが持つディスクリプタの件数。サンプラー付きの画像は影の1つに遠方環境の枝の3つ(拡散照度・鏡面畳込み・
/// 反射率積分表)が加わり、ヘッダの定数バッファ1つとレコード列のストレージバッファ2つが続く。
/// サンプラーを持たない画像は局所可視度の1つであり、こちらは枝によらず必ず在る。
pub(super) struct セット1つの件数 {
    pub(super) サンプラー付き画像: u32,
    pub(super) サンプラー無し画像: u32,
    pub(super) 定数バッファ: u32,
    pub(super) ストレージバッファ: u32,
}

/// 束縛レイアウトの枝ごとの件数。宣言するバインドと件数を1つの判定から導き、プールだけが古くなることを防ぐ。
pub(super) fn セットあたりの件数(束縛レイアウト: 照明束縛レイアウト) -> セット1つの件数 {
    let 遠方環境の画像 = if 束縛レイアウト.遠方環境の画像を結ぶか() {
        3
    } else {
        0
    };
    セット1つの件数 {
        サンプラー付き画像: 1 + 遠方環境の画像,
        サンプラー無し画像: 1,
        定数バッファ: 1,
        ストレージバッファ: 2,
    }
}

pub(super) fn スロット数をu32にする(スロット数: usize) -> u32 {
    u32::try_from(スロット数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"))
}

pub(super) fn 生成する(
    device: &ash::Device,
    スロット数: usize,
    束縛レイアウト: 照明束縛レイアウト,
) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let スロット数 = スロット数をu32にする(スロット数);
    let 件数一式 = セットあたりの件数(束縛レイアウト);
    let プールサイズ一覧 = [
        件数(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 件数一式.サンプラー付き画像 * スロット数),
        件数(vk::DescriptorType::SAMPLED_IMAGE, 件数一式.サンプラー無し画像 * スロット数),
        件数(vk::DescriptorType::UNIFORM_BUFFER, 件数一式.定数バッファ * スロット数),
        件数(vk::DescriptorType::STORAGE_BUFFER, 件数一式.ストレージバッファ * スロット数),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(スロット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn 件数(種別: vk::DescriptorType, 数: u32) -> vk::DescriptorPoolSize {
    vk::DescriptorPoolSize::default().ty(種別).descriptor_count(数)
}
