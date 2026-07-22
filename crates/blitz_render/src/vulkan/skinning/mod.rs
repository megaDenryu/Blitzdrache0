//! GPUスキニング一式(判断44): レスト頂点・スキン属性・スキン行列(フレームインフライト2重)・
//! スキン済み頂点バッファと、コンピュートパイプライン+ディスクリプタ。
//! スキン付きシーンのときのみレンダラーが保持する。バッファは`buffers`、
//! ディスクリプタは`descriptor`、パイプラインは`pipeline`にある。

mod buffers;
mod create;
mod descriptor;
mod pipeline;

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) struct スキニング一式 {
    バッファ: buffers::スキニングバッファ,
    ディスクリプタ: descriptor::スキニングディスクリプタ,
    パイプライン: pipeline::スキニングパイプライン,
    pub(crate) 頂点数: u32,
    ジョイント数: usize,
}

impl スキニング一式 {
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
            panic!(
                "スキン行列数{}がジョイント数{}と一致しない(呼び出し側の配線のバグ)",
                行列一覧.len(),
                self.ジョイント数
            );
        }
        self.バッファ.行列を書き込む(device, フレーム添字, 行列一覧)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
