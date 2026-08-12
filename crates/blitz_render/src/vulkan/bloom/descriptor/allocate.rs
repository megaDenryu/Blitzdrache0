//! 段数に応じたプールとセット群を取り出す局面。呼ばれるのはピラミッドを作り直すたびであり、
//! レイアウトを作る局面(レンダラー生成時の1度だけ)と呼び出し頻度が違う。
//!
//! 宣言ごとに割り当てを分けるのは、1回の割り当てが1つのレイアウトしか受けないためである。

use ash::vk;

use super::光のにじみセット群;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::宣言から作ったセットレイアウト;

pub(in crate::vulkan::bloom) fn 生成する(
    device: &ash::Device,
    単一読みlayout: &宣言から作ったセットレイアウト<1>,
    二読みlayout: &宣言から作ったセットレイアウト<2>,
    段数: usize,
) -> Result<光のにじみセット群, レンダラーエラー> {
    let 拡大段数 = 段数.saturating_sub(1);
    let セット数 = 1 + 拡大段数 * 2;
    let ディスクリプタ数 = 1 + 拡大段数 + 拡大段数 * 2;
    let pool_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(usizeをu32へ(ディスクリプタ数));
    let pool_size一覧 = [pool_size];
    let pool_info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(usizeをu32へ(セット数))
        .pool_sizes(&pool_size一覧);
    // 安全性: deviceは生成済みで有効。
    let pool = unsafe { device.create_descriptor_pool(&pool_info, None)? };

    match 段ごとのセットを取り出す(device, pool, 単一読みlayout, 二読みlayout, 拡大段数) {
        Ok(群) => Ok(群),
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り)
        }
    }
}

/// 単一読みを前処理1つと縮小の段数ぶん、二読みを拡大の段数ぶん取り出す。
fn 段ごとのセットを取り出す(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    単一読みlayout: &宣言から作ったセットレイアウト<1>,
    二読みlayout: &宣言から作ったセットレイアウト<2>,
    拡大段数: usize,
) -> Result<光のにじみセット群, レンダラーエラー> {
    let mut 単一読み一覧 = 単一読みlayout.プールからセットを割り当てる(device, pool, 1 + 拡大段数)?;
    let 拡大set一覧 = 二読みlayout.プールからセットを割り当てる(device, pool, 拡大段数)?;
    let 縮小set一覧 = 単一読み一覧.split_off(1);
    let Some(前処理set) = 単一読み一覧.into_iter().next() else {
        panic!("前処理のセットが1つも割り当てられなかった");
    };
    Ok(光のにじみセット群::束ねる(pool, 前処理set, 縮小set一覧, 拡大set一覧))
}

fn usizeをu32へ(値: usize) -> u32 {
    u32::try_from(値).unwrap_or_else(|_| panic!("ディスクリプタ数がu32に収まらない: {値}"))
}
