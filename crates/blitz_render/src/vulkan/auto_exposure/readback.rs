//! GPU上のヒストグラムと露出状態を検収の観測値へ写す局面。呼ばれるのは検収の実行が求めたフレームだけであり、
//! 本番のフレーム経路は1度も通らない。
//!
//! 注意: 進行中のフレームが同じバッファを書いている最中に読むと中身が定まらないため、呼び出し元はGPUの待機を済ませてから呼ぶ。

use ash::vk;

use super::storage::{ヒストグラムのバイト数, 露出状態のバイト数};
use super::自動露出一式;
use crate::auto_exposure::{ビン数, 自動露出の観測};
use crate::error::レンダラーエラー;
use crate::vulkan::readback::語列を読み戻す;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

impl 自動露出一式 {
    /// 前提: 呼び出し時点でGPUがこの2本のバッファを使用していないこと(device_wait_idle後)。
    pub(crate) fn 観測を読み戻す(
        &self,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
    ) -> Result<自動露出の観測, レンダラーエラー> {
        let ヒストグラム = 語列を読み戻す(
            device,
            メモリプロパティ,
            転送環境,
            self.バッファ.ヒストグラム.handle,
            ヒストグラムのバイト数(),
        )?;
        let 状態 = 語列を読み戻す(device, メモリプロパティ, 転送環境, self.バッファ.露出状態.handle, 露出状態のバイト数)?;
        Ok(観測へ組む(&ヒストグラム, &状態))
    }
}

/// 読み戻した語の列を意味づけへ写す。並びの正本は`shaders/auto_exposure_scale.slang`と`shaders/auto_exposure_state.slang`である。
fn 観測へ組む(ヒストグラム: &[u32], 状態: &[u32]) -> 自動露出の観測 {
    let mut ビンの件数 = [0u32; ビン数];
    ビンの件数.copy_from_slice(&ヒストグラム[..ビン数]);
    自動露出の観測 {
        ビンの件数,
        零以下の件数: ヒストグラム[ビン数],
        非有限の件数: ヒストグラム[ビン数 + 1],
        初期化済み: 状態[0] != 0,
        補正段: f32::from_bits(状態[1]),
        直近の目標補正段: f32::from_bits(状態[3]),
        導出不能フレーム数: 状態[2],
        添字の重み付き総和の下位の語: 状態[4],
        添字の重み付き総和の上位の語: 状態[5],
    }
}
