//! 計測のコンピュートパイプライン。共通の2本と、選ばれた方式の工程だけを作る。全本が同じディスクリプタレイアウトと
//! プッシュ定数の範囲(彩色の区間の2語)を持つパイプラインレイアウトを共有する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::xpbd_solver_bench_probe::XPBDシェーダー一式;
use crate::xpbd_solver_bench_probe::XPBD並列方式;

/// 彩色の区間(開始と本数)をプッシュ定数で運ぶバイト数。
pub(super) const プッシュ定数のバイト数: u32 = 8;

/// 方式ごとの工程のパイプライン。並びはその方式が1反復で積む順である。
pub(super) enum 方式の工程 {
    原子加算 { 拘束: vk::Pipeline, 適用: vk::Pipeline },
    グラフ彩色 { 拘束: vk::Pipeline },
    二段階 { 拘束: vk::Pipeline, 集約: vk::Pipeline },
}

pub(super) struct XPBD計測パイプライン群 {
    pub(super) layout: vk::PipelineLayout,
    pub(super) 積分: vk::Pipeline,
    pub(super) 乗数零化: vk::Pipeline,
    pub(super) 工程: 方式の工程,
}

impl XPBD計測パイプライン群 {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &XPBDシェーダー一式,
        方式: XPBD並列方式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let レイアウト一覧 = [ディスクリプタlayout];
        let プッシュ定数 = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .offset(0)
            .size(プッシュ定数のバイト数)];
        let layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&レイアウト一覧)
            .push_constant_ranges(&プッシュ定数);
        // 安全性: deviceは生成済みで有効。layout_infoは本関数内で構築した値のみを参照する。
        let layout = unsafe { device.create_pipeline_layout(&layout_info, None)? };
        let 仕様一覧: Vec<(&[u8], &std::ffi::CStr)> = 方式の仕様一覧(シェーダー, 方式);
        let mut 生成済み: Vec<vk::Pipeline> = Vec::with_capacity(仕様一覧.len());
        for (spirv, エントリ名) in 仕様一覧 {
            match 確保係.コンピュートパイプラインを生成する(layout, spirv, エントリ名) {
                Ok(handle) => 生成済み.push(handle),
                Err(誤り) => {
                    破棄する(device, layout, &生成済み);
                    return Err(誤り);
                }
            }
        }
        let 工程 = match 方式 {
            XPBD並列方式::原子加算 => 方式の工程::原子加算 {
                拘束: 生成済み[2],
                適用: 生成済み[3],
            },
            XPBD並列方式::グラフ彩色 => 方式の工程::グラフ彩色 { 拘束: 生成済み[2] },
            XPBD並列方式::二段階 => 方式の工程::二段階 {
                拘束: 生成済み[2],
                集約: 生成済み[3],
            },
        };
        Ok(Self {
            layout,
            積分: 生成済み[0],
            乗数零化: 生成済み[1],
            工程,
        })
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        let mut 一覧 = vec![self.積分, self.乗数零化];
        match self.工程 {
            方式の工程::原子加算 { 拘束, 適用 } | 方式の工程::二段階 { 拘束, 集約: 適用 } => 一覧.extend([拘束, 適用]),
            方式の工程::グラフ彩色 { 拘束 } => 一覧.push(拘束),
        }
        破棄する(device, self.layout, &一覧);
    }
}

/// 共通の2本を先頭に、方式の工程を積む順に並べる。
fn 方式の仕様一覧(シェーダー: &XPBDシェーダー一式, 方式: XPBD並列方式) -> Vec<(&[u8], &'static std::ffi::CStr)> {
    let mut 一覧 = vec![
        (シェーダー.積分.コード(), c"integrateMain"),
        (シェーダー.乗数零化.コード(), c"lambdaClearMain"),
    ];
    match 方式 {
        XPBD並列方式::原子加算 => {
            一覧.push((シェーダー.原子加算の拘束.コード(), c"constraintAtomicMain"));
            一覧.push((シェーダー.原子加算の適用.コード(), c"applyAtomicMain"));
        }
        XPBD並列方式::グラフ彩色 => 一覧.push((シェーダー.彩色の拘束.コード(), c"constraintColoredMain")),
        XPBD並列方式::二段階 => {
            一覧.push((シェーダー.二段階の拘束.コード(), c"constraintCandidateMain"));
            一覧.push((シェーダー.二段階の集約.コード(), c"gatherMain"));
        }
    }
    一覧
}

fn 破棄する(device: &ash::Device, layout: vk::PipelineLayout, 一覧: &[vk::Pipeline]) {
    // 安全性: 各ハンドルは呼び出し元が唯一の所有者であり、破棄時点でGPU側の使用が完了している(刻みごとにフェンスで待つ)。
    unsafe {
        for &handle in 一覧 {
            device.destroy_pipeline(handle, None);
        }
        device.destroy_pipeline_layout(layout, None);
    }
}
