//! 布ディスクリプタセットへの10バッファ書き込み。
//! バインディング順はcloth_step.slang冒頭の表(b0=UBO b1=粒子 b2=前位置 b3=介入 b4=隣接
//! b5=セルカウント b6=セル格納 b7=布頂点 b8=スキン済み頂点 b9=アタッチ対応)。

use ash::vk;

use super::buffers::布バッファ;
use crate::vulkan::sync::フレームスロット添字;

/// 前提: setは割り当て済みで、生成直後(GPU未使用)にのみ呼ばれる。
/// スキン済み頂点が無い(吊るし布=アタッチ0件、判断56)場合はb8へ粒子バッファをダミー束縛する
/// (アタッチパスが積まれないため読まれない。レイアウト上の束縛だけが必要)。
pub(super) fn 書く(
    device: &ash::Device,
    set: vk::DescriptorSet,
    バッファ: &布バッファ,
    スキン済み頂点buffer: Option<vk::Buffer>,
    フレーム添字: フレームスロット添字,
) {
    let buffer一覧 = [
        バッファ.定数一覧.スロットのバッファ(フレーム添字),
        バッファ.粒子.バッファ(),
        バッファ.前位置.バッファ(),
        バッファ.介入一覧.スロットのバッファ(フレーム添字),
        バッファ.隣接.バッファ(),
        バッファ.セルカウント.バッファ(),
        バッファ.セル格納.バッファ(),
        バッファ.布頂点.バッファ(),
        スキン済み頂点buffer.unwrap_or(バッファ.粒子.バッファ()),
        バッファ.アタッチ.バッファ(),
    ];
    let 情報一覧: Vec<[vk::DescriptorBufferInfo; 1]> = buffer一覧
        .iter()
        .map(|&buffer| [vk::DescriptorBufferInfo::default().buffer(buffer).range(vk::WHOLE_SIZE)])
        .collect();
    let write一覧: Vec<vk::WriteDescriptorSet<'_>> = 情報一覧
        .iter()
        .enumerate()
        .map(|(binding, 情報)| {
            let 種別 = if binding == 0 {
                vk::DescriptorType::UNIFORM_BUFFER
            } else {
                vk::DescriptorType::STORAGE_BUFFER
            };
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(u32::try_from(binding).unwrap_or_else(|_| panic!("binding番号がu32に収まらない: {binding}")))
                .descriptor_type(種別)
                .buffer_info(情報)
        })
        .collect();
    // 安全性: 呼び出し元の前提のとおりGPU未使用の時点でのみ呼ばれる。
    unsafe { device.update_descriptor_sets(&write一覧, &[]) };
}
