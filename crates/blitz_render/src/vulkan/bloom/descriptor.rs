//! 光のにじみ用ディスクリプタ: 単一読み(binding0のみ)と二読み(binding0+1)の2レイアウトと、
//! 段数に応じたプール+セット群(前処理1・縮小 段数-1・拡大 段数-1)の割り当て。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{宣言した束縛の並び, 束縛番号};

const 標本器つき: vk::DescriptorType = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
const 画素段: vk::ShaderStageFlags = vk::ShaderStageFlags::FRAGMENT;

/// 読み元1枚のセットの宣言。前処理と縮小が使う。
pub(super) const 単一読みの宣言: 宣言した束縛の並び<1> = 宣言した束縛の並び::生成する([(束縛番号::生成する(0), 標本器つき, 画素段)]);

/// 読み元2枚のセットの宣言。拡大が1段小さい結果と同じ段の縮小結果を混ぜるために使う。
pub(super) const 二読みの宣言: 宣言した束縛の並び<2> =
    宣言した束縛の並び::生成する([(束縛番号::生成する(0), 標本器つき, 画素段), (束縛番号::生成する(1), 標本器つき, 画素段)]);

pub(super) struct 光のにじみセット群 {
    pub(super) pool: vk::DescriptorPool,
    pub(super) 前処理set: vk::DescriptorSet,
    pub(super) 縮小set一覧: Vec<vk::DescriptorSet>,
    pub(super) 拡大set一覧: Vec<vk::DescriptorSet>,
}

/// 単一読み(前処理・縮小用)と二読み(拡大用)のレイアウトを作る。失敗時は前者を片付ける。
pub(super) fn レイアウト2種を作る(
    device: &ash::Device,
) -> Result<(vk::DescriptorSetLayout, vk::DescriptorSetLayout), レンダラーエラー> {
    let 単一読み = 読みレイアウトを作る(device, &単一読みの宣言.セットレイアウトの宣言())?;
    match 読みレイアウトを作る(device, &二読みの宣言.セットレイアウトの宣言()) {
        Ok(二読み) => Ok((単一読み, 二読み)),
        Err(誤り) => {
            // 安全性: 単一読みはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_set_layout(単一読み, None) };
            Err(誤り)
        }
    }
}

fn 読みレイアウトを作る(
    device: &ash::Device,
    binding一覧: &[vk::DescriptorSetLayoutBinding<'_>],
) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(binding一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&layout_info, None)? })
}

pub(super) fn 生成する(
    device: &ash::Device,
    単一読みlayout: vk::DescriptorSetLayout,
    二読みlayout: vk::DescriptorSetLayout,
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

    let mut layout一覧 = vec![単一読みlayout; 1 + 拡大段数];
    layout一覧.extend(std::iter::repeat_n(二読みlayout, 拡大段数));
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済み。失敗時はpoolを片付ける。
    match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
        Ok(一覧) => Ok(光のにじみセット群 {
            pool,
            前処理set: 一覧[0],
            縮小set一覧: 一覧[1..1 + 拡大段数].to_vec(),
            拡大set一覧: 一覧[1 + 拡大段数..].to_vec(),
        }),
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り.into())
        }
    }
}

fn usizeをu32へ(値: usize) -> u32 {
    u32::try_from(値).unwrap_or_else(|_| panic!("ディスクリプタ数がu32に収まらない: {値}"))
}
