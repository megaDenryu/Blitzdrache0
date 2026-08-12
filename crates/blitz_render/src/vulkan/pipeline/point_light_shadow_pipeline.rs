//! 点光源の影の記録パイプライン(判断n): 深度のみのグラフィックスパイプライン。頂点シェーダーが位置を
//! その面のライトビュー射影で変換し、画素段は空である。自分のパイプラインレイアウトを抱えたまま持ち回る。
//!
//! 多段影のパイプラインと別に持つのは3つの理由による。プッシュ定数が80バイト(基準原点+面の行列)であり
//! 20バイトの多段影と互換でないこと。ラスタライザの深度の偏りを持たず、偏りを標本する側の世界の長さで掛けること
//! (参照: `crates/blitz_render/src/point_light_shadow/depth_bias.rs`)。頂点段が可視ID列を参照しないことである。
//!
//! 材質を1つも読まないため材質描画族の台帳の外に置く。台帳は材質変種を軸に持つ表であり、
//! 変種を1つも持たないパイプラインをそこへ入れると、材質の軸を読まないパイプラインへ材質の軸が強制される。

mod assemble;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::point_light_shadow_map::点光源の影の形式;
use crate::vulkan::point_light_shadow_push::点光源の影の描画定数;

pub(crate) struct 点光源の影のパイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl 点光源の影のパイプライン {
    /// `ディスクリプタlayout一覧`はset0から順にビューとパス・ジオメトリの2つである。頂点段が読むのは
    /// ジオメトリのセットの個体変換だけであるが、セット番号を1に固定するためset0も宣言する。
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let layout = super::layout::生成する(device, ディスクリプタlayout一覧, 点光源の影の描画定数::プッシュ定数範囲())?;
        match pipelineを生成する(確保係, layout, シェーダー) {
            Ok(handle) => Ok(Self { handle, layout }),
            Err(誤り) => {
                super::layout::破棄する(device, layout);
                Err(誤り)
            }
        }
    }

    /// 前提: レンダラー全体の破棄順を持つ`renderer/destroy.rs`が、GPU待機の済んだ1段として呼ぶ。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handleはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_pipeline(self.handle, None) };
        super::layout::破棄する(device, self.layout);
    }
}

/// シェーダーモジュールの生成から破棄までの局面。パイプラインの生成呼び出しの間だけモジュールが要る。
fn pipelineを生成する(
    確保係: &GPU資源の確保係<'_>,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
) -> Result<vk::Pipeline, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 頂点モジュール = 確保係.シェーダーモジュールを生成する(シェーダー.頂点コード())?;
    let 画素段モジュール = match 確保係.シェーダーモジュールを生成する(シェーダー.画素段コード()) {
        Ok(モジュール) => モジュール,
        Err(誤り) => {
            // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_shader_module(頂点モジュール, None) };
            return Err(誤り);
        }
    };
    let 結果 = assemble::組み立てる(device, 点光源の影の形式, super::描画の標本数, layout, 頂点モジュール, 画素段モジュール);
    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe {
        device.destroy_shader_module(頂点モジュール, None);
        device.destroy_shader_module(画素段モジュール, None);
    }
    結果
}
