//! 布バッファの生成手順。途中失敗時は確保済みを巻き戻す(skinningのbuffers/createと同じ台帳を使う)。

mod input_buffers;

use ash::vk;

use super::布バッファ;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::巻き戻せる確保の台帳;
use crate::vulkan::transfer::ステージング経由の転送係;

use super::super::params;

const セル総数: u64 = 32 * 32 * 32;
const セル容量: u64 = 8;

pub(crate) fn 生成する(
    転送係: ステージング経由の転送係<'_>, 素材: &布素材
) -> Result<布バッファ, レンダラーエラー> {
    let 確保係 = 転送係.確保係();
    let mut 台帳 = 巻き戻せる確保の台帳::始める(確保係);
    let 粒子数 = u64::from(素材.粒子数);
    let ストレージ = vk::BufferUsageFlags::STORAGE_BUFFER;

    let 粒子 = 台帳.積む(転送係.データからデバイスローカルバッファを確保する(&素材.粒子バイト列, ストレージ))?;
    let 前位置 = 台帳.積む(確保係.デバイスローカルバッファを確保する(粒子数 * 16, ストレージ))?;
    let 隣接 = 台帳.積む(転送係.データからデバイスローカルバッファを確保する(&素材.隣接バイト列, ストレージ))?;
    let セルカウント = 台帳.積む(確保係.デバイスローカルバッファを確保する(セル総数 * 4, ストレージ))?;
    let セル格納 = 台帳.積む(確保係.デバイスローカルバッファを確保する(セル総数 * セル容量 * 4, ストレージ))?;
    let 布頂点 = 台帳.積む(確保係.デバイスローカルバッファを確保する(粒子数 * 48, ストレージ | vk::BufferUsageFlags::VERTEX_BUFFER))?;

    let (インデックス, アタッチ) = input_buffers::生成する(&mut 台帳, 転送係, 素材, ストレージ)?;

    let 介入初期値 =
        vec![0u8; usize::try_from(params::介入上限件数).unwrap_or_else(|_| panic!("介入上限件数がusizeに収まらない")) * 32];
    let 介入一覧 =
        台帳.フレームスロットごとに積む(確保係.フレームスロットごとのホスト可視バッファを確保して書き込む(&介入初期値, ストレージ))?;
    let 定数一覧 = 台帳.フレームスロットごとに積む(確保係.フレームスロットごとのホスト可視バッファを確保して書き込む(
        &[0u8; params::バイト長],
        vk::BufferUsageFlags::UNIFORM_BUFFER,
    ))?;

    Ok(布バッファ {
        粒子,
        前位置,
        隣接,
        セルカウント,
        セル格納,
        布頂点,
        インデックス,
        アタッチ,
        介入一覧,
        定数一覧,
    })
}
