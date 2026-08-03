//! 反射率積分表の生成コンピュートが束縛するディスクリプタ。binding0が書き込み先の2次元画像だけである。
//!
//! 派生表現ディスクリプタと別の型にするのは、この表が遠方環境を1テクセルも読まないためである。読まない資源を
//! レイアウトへ載せると、束縛し忘れても通ってしまう組み合わせが生まれる。
//!
//! 注意: 番号は`shaders/brdf_integration.slang`の`vk::binding`と一致させる。
//! 注意: 書き込み先のレイアウトはGENERALである。レンダーグラフの画像用途「コンピュート書き」が同じレイアウトへ
//! 遷移させており、ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。

use ash::vk;

use crate::error::レンダラーエラー;

const 書き込み先の番号: u32 = 0;
const 書き込み先の種別: vk::DescriptorType = vk::DescriptorType::STORAGE_IMAGE;

pub(super) struct 反射率積分表ディスクリプタ {
    pub(super) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set: vk::DescriptorSet,
}

impl 反射率積分表ディスクリプタ {
    pub(super) fn 生成する(device: &ash::Device, 書き込み先: vk::ImageView) -> Result<Self, レンダラーエラー> {
        let layout = レイアウトを作る(device)?;
        let pool = match プールを作る(device) {
            Ok(pool) => pool,
            Err(誤り) => return Err(レイアウトを片付けて返す(device, layout, 誤り)),
        };
        let layout一覧 = [layout];
        let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
        // 安全性: pool・layoutは直前に生成済みで有効。
        let set一覧 = match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                return Err(レイアウトを片付けて返す(device, layout, 誤り.into()));
            }
        };
        let Some(&set) = set一覧.first() else {
            panic!("反射率積分表のディスクリプタセットが1つも割り当てられなかった");
        };
        書き込む(device, set, 書き込み先);
        Ok(Self { layout, pool, set })
    }

    pub(super) fn set(&self) -> vk::DescriptorSet {
        self.set
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。layout・poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [vk::DescriptorSetLayoutBinding::default()
        .binding(書き込み先の番号)
        .descriptor_type(書き込み先の種別)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::COMPUTE)];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default().ty(書き込み先の種別).descriptor_count(1)];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn 書き込む(device: &ash::Device, set: vk::DescriptorSet, 書き込み先: vk::ImageView) {
    let 情報 = [vk::DescriptorImageInfo::default()
        .image_view(書き込み先)
        .image_layout(vk::ImageLayout::GENERAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(書き込み先の番号)
        .descriptor_type(書き込み先の種別)
        .image_info(&情報)];
    // 安全性: setは割当済み、画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

fn レイアウトを片付けて返す(
    device: &ash::Device, layout: vk::DescriptorSetLayout, 誤り: レンダラーエラー
) -> レンダラーエラー {
    // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
    unsafe { device.destroy_descriptor_set_layout(layout, None) };
    誤り
}
