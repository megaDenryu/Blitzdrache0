//! GPUスキニング一式(判断44): レスト頂点・スキン属性・スキン行列(フレームインフライト2重)・
//! スキン済み頂点バッファと、コンピュートパイプライン+ディスクリプタ。
//! スキン付きシーンのときのみレンダラーが保持する。バッファは`buffers`、
//! ディスクリプタは`descriptor`、パイプラインは`pipeline`にある。

mod buffers;
mod descriptor;
mod pipeline;

use ash::vk;

use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct スキニング一式 {
    バッファ: buffers::スキニングバッファ,
    ディスクリプタ: descriptor::スキニングディスクリプタ,
    パイプライン: pipeline::スキニングパイプライン,
    pub(crate) 頂点数: u32,
    ジョイント数: usize,
}

impl スキニング一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        頂点一覧: &[頂点],
        素材: &スキンメッシュ素材,
        シェーダー: &コンピュートシェーダー,
    ) -> Result<Self, レンダラーエラー> {
        if 素材.属性一覧().len() != 頂点一覧.len() {
            return Err(レンダラーエラー::スキン属性数不一致 {
                属性数: 素材.属性一覧().len(),
                頂点数: 頂点一覧.len(),
            });
        }
        let 頂点数 = u32::try_from(頂点一覧.len())
            .unwrap_or_else(|_| panic!("頂点数がu32に収まらない: {}", 頂点一覧.len()));

        let バッファ = buffers::生成する(device, メモリプロパティ, 転送環境, 頂点一覧, 素材)?;
        let ディスクリプタ = match descriptor::生成する(device, &バッファ) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let パイプライン = match pipeline::生成する(device, ディスクリプタ.layout, シェーダー.コード()) {
            Ok(パイプライン) => パイプライン,
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self { バッファ, ディスクリプタ, パイプライン, 頂点数, ジョイント数: 素材.ジョイント数() })
    }

    /// スキン済み頂点バッファ(シーン/シャドウパスが頂点入力として読む)。
    pub(crate) fn 出力バッファ(&self) -> vk::Buffer {
        self.バッファ.出力.0
    }

    pub(crate) fn パイプラインhandle(&self) -> vk::Pipeline {
        self.パイプライン.handle
    }

    pub(crate) fn パイプラインlayout(&self) -> vk::PipelineLayout {
        self.パイプライン.layout
    }

    pub(crate) fn set(&self, フレーム添字: usize) -> vk::DescriptorSet {
        self.ディスクリプタ.set一覧[フレーム添字]
    }

    /// このフレームのスキン行列(列優先4x4)を書き込む。呼び出しはフェンス待ち後(判断24と同じ規律)。
    pub(crate) fn 行列を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: usize,
        行列一覧: &[[f32; 16]],
    ) -> Result<(), レンダラーエラー> {
        if 行列一覧.len() != self.ジョイント数 {
            panic!("スキン行列数{}がジョイント数{}と一致しない(呼び出し側の配線のバグ)", 行列一覧.len(), self.ジョイント数);
        }
        self.バッファ.行列を書き込む(device, フレーム添字, 行列一覧)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
