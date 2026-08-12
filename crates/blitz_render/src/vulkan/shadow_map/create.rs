//! シャドウマップの配列画像・メモリ・画像ビュー群・比較サンプラーの確保。
//! 呼ばれるのはレンダラー生成時の1回だけであり、途中で失敗したらそれまでに作ったハンドルをその場で逆順に破棄する。

mod image;

use ash::vk;

use self::image::{画像を作る, 距離区分ビューを作る, 配列ビューを作る};
use super::sampler::比較サンプラーを作る;
use super::シャドウマップ;
use crate::cascade::{影の一辺解像度, 距離区分数};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>, 一辺: 影の一辺解像度
) -> Result<シャドウマップ, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = 画像を作る(確保係, 一辺)?;
    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(画像, GPUメモリ用途::描画画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match 続きを生成する(確保係, 画像) {
        Ok((配列ビュー, 距離区分別のビュー一覧, sampler)) => Ok(シャドウマップ {
            一辺,
            画像,
            配列ビュー,
            距離区分別のビュー一覧,
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
    確保係: &GPU資源の確保係<'_>,
    画像: vk::Image,
) -> Result<(vk::ImageView, [vk::ImageView; 距離区分数], vk::Sampler), レンダラーエラー> {
    let mut 作成済みビュー: Vec<vk::ImageView> = Vec::with_capacity(距離区分数 + 1);
    let device = 確保係.論理デバイス();
    let 結果 = ビュー群とサンプラーを作る(確保係, 画像, &mut 作成済みビュー);
    if 結果.is_err() {
        for &ビュー in &作成済みビュー {
            // 安全性: 作成済みビューはこの関数が作ったもので、失敗時は誰も参照していない。
            unsafe { device.destroy_image_view(ビュー, None) };
        }
    }
    結果
}

fn ビュー群とサンプラーを作る(
    確保係: &GPU資源の確保係<'_>,
    画像: vk::Image,
    作成済みビュー: &mut Vec<vk::ImageView>,
) -> Result<(vk::ImageView, [vk::ImageView; 距離区分数], vk::Sampler), レンダラーエラー> {
    let 配列ビュー = 配列ビューを作る(確保係, 画像)?;
    作成済みビュー.push(配列ビュー);
    let mut 距離区分別のビュー一覧 = [vk::ImageView::null(); 距離区分数];
    for (添字, 保存先) in 距離区分別のビュー一覧.iter_mut().enumerate() {
        let 層 = u32::try_from(添字).unwrap_or_else(|_| panic!("距離区分の添字がu32に収まらない: {添字}"));
        let ビュー = 距離区分ビューを作る(確保係, 画像, 層)?;
        作成済みビュー.push(ビュー);
        *保存先 = ビュー;
    }
    let sampler = 比較サンプラーを作る(確保係)?;
    Ok((配列ビュー, 距離区分別のビュー一覧, sampler))
}
