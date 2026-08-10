//! 点光源の影の立方体配列の画像・メモリ・画像ビュー群・比較サンプラーの確保。
//! 呼ばれるのはレンダラー生成時の1回だけであり、途中で失敗したらそれまでに作ったハンドルをその場で逆順に破棄する。

mod image;

use ash::vk;

use self::image::{メモリを確保して結びつける, 層ビューを作る, 画像を作る, 立方体配列ビューを作る};
use super::sampler::比較サンプラーを作る;
use super::{点光源の影の層数, 点光源の影の立方体配列};
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
) -> Result<点光源の影の立方体配列, レンダラーエラー> {
    let 画像 = 画像を作る(device)?;
    let memory = match メモリを確保して結びつける(device, メモリプロパティ, 画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match 続きを生成する(device, 画像) {
        Ok((立方体配列ビュー, 層別のビュー一覧, sampler)) => Ok(点光源の影の立方体配列 {
            画像,
            立方体配列ビュー,
            層別のビュー一覧,
            sampler,
            memory,
        }),
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。ビュー群は`続きを生成する`が後始末済み。
            unsafe { device.destroy_image(画像, None) };
            device.メモリを解放する(memory);
            Err(誤り)
        }
    }
}

/// ビュー群とサンプラーを作る。1つでも失敗したら、この関数が作ったハンドルだけを逆順に破棄して失敗を返す。
fn 続きを生成する(
    device: &GPUデバイス,
    画像: vk::Image,
) -> Result<(vk::ImageView, Vec<vk::ImageView>, vk::Sampler), レンダラーエラー> {
    let 層数 = 点光源の影の層数();
    let 個数 = usize::try_from(層数).unwrap_or_else(|_| panic!("点光源の影の層数がusizeに収まらない: {層数}"));
    let mut 作成済みビュー: Vec<vk::ImageView> = Vec::with_capacity(個数 + 1);
    let 結果 = ビュー群とサンプラーを作る(device, 画像, &mut 作成済みビュー);
    if 結果.is_err() {
        for &ビュー in &作成済みビュー {
            // 安全性: 作成済みビューはこの関数が作ったもので、失敗時は誰も参照していない。
            unsafe { device.destroy_image_view(ビュー, None) };
        }
    }
    結果
}

fn ビュー群とサンプラーを作る(
    device: &GPUデバイス,
    画像: vk::Image,
    作成済みビュー: &mut Vec<vk::ImageView>,
) -> Result<(vk::ImageView, Vec<vk::ImageView>, vk::Sampler), レンダラーエラー> {
    let 立方体配列ビュー = 立方体配列ビューを作る(device, 画像)?;
    作成済みビュー.push(立方体配列ビュー);
    let mut 層別のビュー一覧 = Vec::with_capacity(作成済みビュー.capacity());
    for 層 in 0..点光源の影の層数() {
        let ビュー = 層ビューを作る(device, 画像, 層)?;
        作成済みビュー.push(ビュー);
        層別のビュー一覧.push(ビュー);
    }
    let sampler = 比較サンプラーを作る(device)?;
    Ok((立方体配列ビュー, 層別のビュー一覧, sampler))
}
