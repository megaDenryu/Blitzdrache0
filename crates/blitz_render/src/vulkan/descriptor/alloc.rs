//! ディスクリプタセットの割り当て。担当するのは、同じレイアウトのセットを指定の数だけプールから取り出し、
//! 要求と実際の件数の食い違いをその場で落とすことである。書き込む内容は各セットの役割のモジュールが持つ。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
    セット数: usize,
) -> Result<Vec<vk::DescriptorSet>, レンダラーエラー> {
    if セット数 == 0 {
        return Ok(Vec::new());
    }
    let layout一覧 = vec![layout; セット数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    if set一覧.len() != セット数 {
        panic!("allocate_descriptor_setsが要求{セット数}個と異なる件数を返した");
    }
    Ok(set一覧)
}
