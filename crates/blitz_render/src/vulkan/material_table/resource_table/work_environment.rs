//! 材質資源表の操作が供給元へ渡す借り物の束。担当するのは、論理デバイス・物理デバイスへの問い合わせ・メモリプロパティ・
//! 転送実行環境・セットレイアウトの5つを1つの値として運ぶことである。
//!
//! 5つを1つにまとめるのは、これらを使う操作が5箇所(起動・束の追加・束の解除・シーンの差し替え・破棄)あり、
//! 引数として並べると1つ取り違えても型が通るためである。呼び出し元がレンダラーのフィールドから直に組み立てるのは、
//! 材質資源表そのものを可変で借りる操作と借用が重ならないようにするためである。

use ash::vk;

use crate::vulkan::descriptor::シーンセットレイアウト一式;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

#[derive(Clone, Copy)]
pub(crate) struct 材質資源の作業環境<'環境> {
    pub(crate) device: &'環境 GPUデバイス,
    pub(crate) 問い合わせ: 物理デバイス問い合わせ<'環境>,
    pub(crate) メモリプロパティ: &'環境 vk::PhysicalDeviceMemoryProperties,
    pub(crate) 転送環境: &'環境 転送実行環境,
    pub(crate) セットレイアウト: &'環境 シーンセットレイアウト一式,
}
