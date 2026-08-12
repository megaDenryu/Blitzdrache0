//! set3(照明問い合わせのセット)のレイアウトと、そこへ資源を結ぶ操作の入口。触れるのはシャドウマップの比較サンプラー(binding0)・
//! ヘッダの定数バッファ(binding1)・方向光レコード列(binding2)・局所光レコード列(binding3)の4つと、両方の契約が持つ
//! 局所可視度の画像(binding7、`local_visibility`が担う)とクラスタ格子の2本(binding8と9、`cluster_grid`が担う)と
//! 点光源の影の立方体配列(binding10、`point_light_shadow_map`が担う)、
//! 遠方環境の枝だけが持つ3つ(binding4から6、`distant_environment`が担う)である。
//! 同じセットへ置くのは、どれも寿命がフレーム×ビューであり、直接光と直接影が同じ問い合わせ契約に属するためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「照明問い合わせ資源のGPU境界」)。番号の正本は`shaders/lighting_query.slang`の宣言である。
//! 割り当て済みのセットへ現物を書き込む型は`set_write`が持つ。呼ばれるのが生成時とスワップチェーン再構築の直後だけであり、
//! レイアウトを決める局面と呼ばれる頻度が違うためである。

mod allocated_set;
mod buffer_group;
pub(crate) mod cluster_grid;
pub(crate) mod distant_environment;
pub(crate) mod local_visibility;
pub(crate) mod point_light_shadow_map;
mod pool;
mod set_write;

use ash::vk;

pub(crate) use allocated_set::照明問い合わせの割り当て済みセット;
pub(crate) use buffer_group::照明問い合わせのバッファ組;
pub(crate) use pool::照明問い合わせのディスクリプタプール;
pub(crate) use set_write::照明問い合わせのセットの書き込み先;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::束縛番号;
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;

pub(crate) const シャドウマップの束縛番号: 束縛番号 = 束縛番号::生成する(0);
pub(crate) const ヘッダの束縛番号: 束縛番号 = 束縛番号::生成する(1);
pub(crate) const 方向光列の束縛番号: 束縛番号 = 束縛番号::生成する(2);
pub(crate) const 局所光列の束縛番号: 束縛番号 = 束縛番号::生成する(3);

/// 束縛レイアウトの枝ごとに宣言するバインドの並びを変える。定数近似の枝へ未使用のダミー束縛を強制しない。
pub(super) fn レイアウトを生成する(
    device: &ash::Device,
    束縛レイアウト: 照明束縛レイアウト,
) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let mut バインド一覧 = vec![
        画素段のバインド(シャドウマップの束縛番号, vk::DescriptorType::COMBINED_IMAGE_SAMPLER),
        画素段のバインド(ヘッダの束縛番号, vk::DescriptorType::UNIFORM_BUFFER),
        画素段のバインド(方向光列の束縛番号, vk::DescriptorType::STORAGE_BUFFER),
        画素段のバインド(局所光列の束縛番号, vk::DescriptorType::STORAGE_BUFFER),
    ];
    if 束縛レイアウト.遠方環境の画像を結ぶか() {
        バインド一覧.extend(distant_environment::バインド一覧());
    }
    バインド一覧.push(local_visibility::バインド());
    バインド一覧.extend(cluster_grid::バインド一覧());
    バインド一覧.push(point_light_shadow_map::バインド());
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

fn 画素段のバインド(番号: 束縛番号, 種別: vk::DescriptorType) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(番号.gpu境界値())
        .descriptor_type(種別)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
