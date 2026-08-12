//! 資源表世代1つぶんの束縛先。担当するのは、その世代の材質レコードのストレージバッファと材質のセットを確保し、
//! 退役のときに1組として解放することである。
//!
//! ディスクリプタプールを世代ごとに1つ持つのは、世代の退役がプール1つの破棄で完結するためである。プールを世代で共有すると、
//! どのセットがどの世代のものかを別に管理し、退役の順番と解放の順番を合わせ続けることになる。
//! 注意: 表の要素数はレイアウトの容量ぶん確保する。実際に書き込むのはその世代の常駐枚数までであり、残りは部分束縛で残す。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::材質のセットの書き込み先;
use crate::vulkan::material_record::材質レコードバッファ;
use crate::vulkan::texture::テクスチャ;
use crate::vulkan::tracked_device::GPUデバイス;

use crate::vulkan::material_table::generation_record::世代内材質レコード;
use crate::vulkan::material_table::resource_table::材質資源の作業環境;

pub(in crate::vulkan::material_table) struct 世代束縛資源 {
    レコードバッファ: 材質レコードバッファ,
    pool: vk::DescriptorPool,
    材質のセット: vk::DescriptorSet,
}

impl 世代束縛資源 {
    pub(in crate::vulkan::material_table) fn 生成する(
        環境: 材質資源の作業環境<'_>,
        画像集合: &[テクスチャ],
        レコード列: &[世代内材質レコード],
    ) -> Result<Self, レンダラーエラー> {
        let レコードバッファ = 材質レコードバッファ::生成する(環境.転送係, レコード列)?;
        let pool = match プールを生成する(環境) {
            Ok(値) => 値,
            Err(誤り) => {
                レコードバッファ.破棄する(環境.論理デバイス());
                return Err(誤り);
            }
        };
        match セットを割り当てて結ぶ(環境, pool, &レコードバッファ, 画像集合) {
            Ok(材質のセット) => Ok(Self {
                レコードバッファ,
                pool,
                材質のセット,
            }),
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { 環境.論理デバイス().destroy_descriptor_pool(pool, None) };
                レコードバッファ.破棄する(環境.論理デバイス());
                Err(誤り)
            }
        }
    }

    pub(in crate::vulkan::material_table) fn 材質のセット(&self) -> vk::DescriptorSet {
        self.材質のセット
    }

    /// 注意: プールの破棄がセットの解放を暗黙に行う。
    /// 前提: この世代を束縛した全フレームのフェンス通過を呼び出し元が確かめてから呼ぶ。
    pub(in crate::vulkan::material_table) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.レコードバッファ.破棄する(device);
    }
}

fn プールを生成する(環境: 材質資源の作業環境<'_>) -> Result<vk::DescriptorPool, レンダラーエラー> {
    // 固定サンプラーのbindingも割り当ての対象であるため、書き込まなくてもプールに枠が要る。
    let プールサイズ一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::SAMPLED_IMAGE)
            .descriptor_count(環境.セットレイアウト.材質テクスチャ表容量().枚数()),
        vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLER).descriptor_count(1),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { 環境.論理デバイス().create_descriptor_pool(&create_info, None)? })
}

fn セットを割り当てて結ぶ(
    環境: 材質資源の作業環境<'_>,
    pool: vk::DescriptorPool,
    レコードバッファ: &材質レコードバッファ,
    画像集合: &[テクスチャ],
) -> Result<vk::DescriptorSet, レンダラーエラー> {
    let セット = 環境.セットレイアウト.材質のセットを1つ割り当てる(環境.論理デバイス(), pool)?;
    材質のセットの書き込み先::生成する(環境.論理デバイス(), セット).世代の資源を結ぶ(レコードバッファ, 画像集合);
    Ok(セット)
}
