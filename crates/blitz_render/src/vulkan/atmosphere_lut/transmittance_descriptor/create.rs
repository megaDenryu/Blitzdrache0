//! 透過率ディスクリプタの生成局面。レイアウト・プール・セットの割当と、束縛先を指す内容の書き込みを行う。
//! 呼ばれるのは大気のベイク済み画像一式の組み立て時の1回だけであり、以降のフレームはセットを参照するだけである。

use ash::vk;

use super::透過率ディスクリプタ;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号, 結ぶ現物
};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

const 宣言: 宣言した束縛の並び<2> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::UNIFORM_BUFFER, vk::ShaderStageFlags::COMPUTE),
    (束縛番号::生成する(1), vk::DescriptorType::STORAGE_IMAGE, vk::ShaderStageFlags::COMPUTE),
]);

pub(super) fn 生成する(
    device: &ash::Device,
    シェーダー定数一覧: [vk::Buffer; 進行中フレーム数],
    書き込み先: vk::ImageView,
) -> Result<透過率ディスクリプタ, レンダラーエラー> {
    let layout = レイアウトを作る(device)?;
    let pool = match プールを作る(device) {
        Ok(pool) => pool,
        Err(誤り) => {
            layout.破棄する(device);
            return Err(誤り);
        }
    };
    let set一覧 = match layout.進行中フレームスロットごとのセットを割り当てる(device, pool) {
        Ok(set一覧) => set一覧,
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            layout.破棄する(device);
            return Err(誤り);
        }
    };
    for 添字 in フレームスロット添字::全スロット() {
        書き込む(device, &set一覧[添字.配列添字()], シェーダー定数一覧[添字.配列添字()], 書き込み先);
    }
    Ok(透過率ディスクリプタ { layout, pool, set一覧 })
}

fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<2>, レンダラーエラー> {
    宣言.セットレイアウトを確保する(device)
}

fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = descriptor_common::セット数();
    let プールサイズ一覧 = 宣言.プールの内訳(セット数);
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

/// 注意: ストレージ画像のレイアウトはGENERALである。レンダーグラフの画像用途「コンピュート書き」が同じレイアウトへ遷移させており、
/// ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。
fn 書き込む(
    device: &ash::Device, セット: &宣言から割り当てたセット<2>, シェーダー定数: vk::Buffer, 書き込み先: vk::ImageView
) {
    セット.書き込み先(device).並びの位置ごとに結ぶ([
        結ぶ現物::バッファ全体(シェーダー定数),
        結ぶ現物::サンプラー無しの画像 {
            ビュー: 書き込み先,
            レイアウト: vk::ImageLayout::GENERAL,
        },
    ]);
}
