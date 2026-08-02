//! シャドウパイプライン(判断35): 深度のみのグラフィックスパイプライン。
//! 頂点シェーダーが位置をライトビュー射影変換し、画素段シェーダーは空。
//! 束縛するのはビューとパスのセット(多段影定数)とジオメトリのセット(個体レコード・可視ID列)の2つだけであり、
//! 材質のセットも照明問い合わせのセットも持たない(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

mod assemble;
mod create;
mod finish;
mod vertex_input;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shadow_map::シャドウマップ形式;
use crate::vulkan::shadow_push;

pub(super) use create::生成する as pipelineを生成する;

pub(crate) struct シャドウパイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl シャドウパイプライン {
    /// 自分のパイプラインレイアウトを抱えたまま持ち回る布のシャドウ経路だけが使う。
    /// 材質描画族のシャドウはレイアウトを台帳が族ごとに1つ持つため、この入口を通らない。
    /// `ディスクリプタlayout一覧` はset0から順に並べたビューとパス・ジオメトリの2つである。
    pub(crate) fn 生成する(
        device: &ash::Device,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let layout = super::layout::生成する(device, ディスクリプタlayout一覧, shadow_push::プッシュ定数範囲())?;
        match create::生成する(device, シャドウマップ形式, super::描画の標本数, layout, シェーダー) {
            Ok(handle) => Ok(Self { handle, layout }),
            Err(誤り) => {
                super::layout::破棄する(device, layout);
                Err(誤り)
            }
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handleはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_pipeline(self.handle, None) };
        super::layout::破棄する(device, self.layout);
    }
}
