//! パス別GPU計測用のタイムスタンプクエリプール。進行中フレームごとに
//! 1プール確保し(判断30)、パス前後2発ぶんの容量(`クエリ数上限`)を持つ。

use ash::vk;

use super::クエリ数上限;
use crate::error::レンダラーエラー;
use crate::vulkan::sync::進行中フレーム数;

pub(super) fn 進行中フレーム別のタイムスタンプクエリプールを生成する(
    device: &ash::Device,
) -> Result<[vk::QueryPool; 進行中フレーム数], レンダラーエラー> {
    let mut プール一覧 = [vk::QueryPool::null(); 進行中フレーム数];
    for 添字 in 0..進行中フレーム数 {
        let create_info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::TIMESTAMP)
            .query_count(クエリ数上限);
        // 安全性: deviceは生成済みで有効。
        match unsafe { device.create_query_pool(&create_info, None) } {
            Ok(pool) => プール一覧[添字] = pool,
            Err(誤り) => {
                // 安全性: これまでに生成済みのプールはこのスコープの唯一の所有者で、以降使用しない。
                for &生成済み in &プール一覧[..添字] {
                    unsafe { device.destroy_query_pool(生成済み, None) };
                }
                return Err(誤り.into());
            }
        }
    }
    Ok(プール一覧)
}
