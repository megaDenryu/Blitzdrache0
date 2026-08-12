//! 全画面三角形で1枚の絵を描くパイプラインと、そのパイプラインだけが使うパイプラインレイアウトの組。
//! 頂点入力なし・深度なし・ブレンドなし・動的ビューポート/シザーの固定機能を共有し、
//! 画素段エントリ名とプッシュ定数バイト数(0なら範囲なし)だけがパスごとに異なる(明るさの圧縮・光のにじみが共用)。
//!
//! レイアウトを一緒に持つのは、この族ではパイプライン1本につきレイアウトが1つであり、寿命も所有者も同じだからである。
//! 族ごとに1つのレイアウトを共有する材質描画族は、この型を通らず`pipeline_ledger::layouts`が持つ。

mod fixed_function;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

/// 前提: 全ポストプロセスシェーダーの頂点エントリはこの名前で統一する(tonemap.slang/bloom.slang)。
const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";

pub(crate) struct 全画面パスのパイプライン {
    handle: vk::Pipeline,
    layout: vk::PipelineLayout,
}

impl 全画面パスのパイプライン {
    pub(crate) fn 組み立てる(
        確保係: &GPU資源の確保係<'_>,
        カラー形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
        画素段エントリ名: &std::ffi::CStr,
        プッシュ定数バイト数: u32,
    ) -> Result<Self, レンダラーエラー> {
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

        let 結果 = fixed_function::組み立てる(
            device,
            カラー形式,
            ディスクリプタlayout,
            頂点モジュール,
            頂点エントリ名,
            画素段モジュール,
            画素段エントリ名,
            プッシュ定数バイト数,
        );

        // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
        unsafe {
            device.destroy_shader_module(頂点モジュール, None);
            device.destroy_shader_module(画素段モジュール, None);
        }
        結果
    }

    /// `create_graphics_pipelines`の生成結果から組を作り、失敗していたらレイアウトをその場で片付ける。ブレンドや深度の要否が違う全画面パス(空中遠近合成・時間再構成)も、失敗時の後始末は同じであるためこの口を通る。
    pub(crate) fn 生成結果から取り出す(
        device: &ash::Device,
        layout: vk::PipelineLayout,
        生成結果: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
    ) -> Result<Self, レンダラーエラー> {
        match 生成結果 {
            Ok(一覧) => {
                let Some(&handle) = 一覧.first() else {
                    panic!("create_graphics_pipelinesが成功したのにパイプラインが0本だった(Vulkan実装の契約違反)");
                };
                Ok(Self { handle, layout })
            }
            Err((_, 誤り)) => {
                // 安全性: パイプライン生成に失敗したため、layoutを参照するパイプラインは存在しない。
                unsafe { device.destroy_pipeline_layout(layout, None) };
                Err(誤り.into())
            }
        }
    }

    pub(crate) const fn パイプラインのハンドル(&self) -> vk::Pipeline {
        self.handle
    }

    pub(crate) const fn パイプラインレイアウトのハンドル(&self) -> vk::PipelineLayout {
        self.layout
    }

    /// 前提: 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: パイプラインとレイアウトはSelfが唯一の所有者である。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
