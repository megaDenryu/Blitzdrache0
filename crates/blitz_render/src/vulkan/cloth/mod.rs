//! 布シミュレーション一式(判断52〜54): バッファ群・統一ディスクリプタ・コンピュート9本・
//! 布描画パイプライン。`--cloth`指定のスキン付きシーンのときのみレンダラーが保持する。
//! 生成手順は`create`、描画入力の組み立ては`inputs`にある。

mod buffers;
mod create;
mod descriptor;
mod inputs;
mod params;
mod pipelines;
mod write;

use crate::error::レンダラーエラー;
use crate::frame_input::布フレーム入力;
use crate::vulkan::pipeline::パイプライン;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use create::生成する as 布一式を生成する;

pub(crate) struct 布一式 {
    バッファ: buffers::布バッファ,
    ディスクリプタ: descriptor::布ディスクリプタ,
    パイプライン群: pipelines::布パイプライン群,
    pub(crate) 描画パイプライン: パイプライン,
    固定部: params::固定部,
    pub(crate) インデックス数: u32,
}

impl 布一式 {
    /// 前提: 呼び出しはフェンス待ち後(判断24と同じ規律)。介入バイト列の長さも検証する。
    pub(crate) fn フレーム入力を書き込む(
        &self,
        device: &GPUデバイス,
        フレーム添字: フレームスロット添字,
        入力: &布フレーム入力,
    ) -> Result<(), レンダラーエラー> {
        if 入力.介入件数 > params::介入上限件数
            || 入力.介入バイト列.len() != usize::try_from(入力.介入件数 * 32).unwrap_or_else(|_| panic!("介入バイト長がusizeに収まらない"))
        {
            return Err(レンダラーエラー::布介入不正 {
                件数: 入力.介入件数,
                バイト長: 入力.介入バイト列.len(),
            });
        }
        if !入力.介入バイト列.is_empty() {
            self.バッファ.介入を書き込む(device, フレーム添字, &入力.介入バイト列)?;
        }
        self.バッファ
            .定数を書き込む(device, フレーム添字, &params::バイト列にする(&self.固定部, 入力))
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.描画パイプライン.破棄する(device);
        self.パイプライン群.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
