//! 自動露出が持つ3本のバッファ。担当するのは、3本それぞれの中身の意味と大きさを決め、生成の順と初期値を守ることである。
//!
//! 境界の列は起動時に1度だけ書いて以降変えない。ヒストグラムは毎フレーム消去して数え直す。
//! 露出状態は世界の1本の時系列であり、生成時に全要素0(未初期化)へ落として以降はGPUだけが書く。
//! 注意: 露出状態を進行中フレームのスロット別に分けてはならない。分けるとNフレームがN-2フレームの状態を読む別系列ができる。

use super::storage_buffer::記憶バッファ;
use crate::auto_exposure::ビン数;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::ステージング経由の転送係;

/// ヒストグラムのバッファが持つ要素数。ビンの本数に、0以下の枠と非有限の枠を足した数である。
/// 注意: `shaders/auto_exposure_scale.slang`の`histogramSlotCount`と同じ値でなければならない。この式そのものは
/// `cargo xtask conform`の定数一致検査が読めないため、検査はビンの本数と同slangの`nonPositiveSlotIndex`の一致で代わりを果たす。
pub(crate) const ヒストグラムの要素数: usize = ビン数 + 2;
/// ヒストグラムのバッファのバイト数。1要素は32ビットの件数である。
pub(crate) fn ヒストグラムのバイト数() -> u64 {
    u64::try_from(ヒストグラムの要素数 * 4).unwrap_or_else(|_| panic!("ヒストグラムのバイト数がu64に収まらない"))
}
/// 露出状態1件のバイト数。`shaders/auto_exposure_state.slang`の`ExposureState`(4バイトの要素6つ)と一致させる。
pub(crate) const 露出状態のバイト数: u64 = 24;

pub(crate) struct 自動露出のバッファ一式 {
    pub(crate) 境界: 記憶バッファ,
    pub(crate) ヒストグラム: 記憶バッファ,
    pub(crate) 露出状態: 記憶バッファ,
}

impl 自動露出のバッファ一式 {
    pub(crate) fn 生成する(
        転送係: ステージング経由の転送係<'_>, 境界の線形輝度: &[f32; ビン数]
    ) -> Result<Self, レンダラーエラー> {
        let device = 転送係.論理デバイス();
        let バイト列: Vec<u8> = 境界の線形輝度.iter().flat_map(|値| 値.to_le_bytes()).collect();
        let 境界 = 記憶バッファ::書き込んで確保する(転送係, &バイト列)?;
        let ヒストグラム = match 記憶バッファ::確保する(転送係.確保係(), ヒストグラムのバイト数()) {
            Ok(バッファ) => バッファ,
            Err(誤り) => {
                境界.破棄する(device);
                return Err(誤り);
            }
        };
        let 一式 = match 露出状態を用意する(転送係) {
            Ok(露出状態) => Self {
                境界,
                ヒストグラム,
                露出状態,
            },
            Err(誤り) => {
                ヒストグラム.破棄する(device);
                境界.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(一式)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.露出状態.破棄する(device);
        self.ヒストグラム.破棄する(device);
        self.境界.破棄する(device);
    }
}

/// 確保に続けて未初期化(全要素0)を書き込む。0を書き損ねると最初のフレームが不定の補正段を読む。
fn 露出状態を用意する(転送係: ステージング経由の転送係<'_>) -> Result<記憶バッファ, レンダラーエラー> {
    let device = 転送係.論理デバイス();
    let バッファ = 記憶バッファ::確保する(転送係.確保係(), 露出状態のバイト数)?;
    if let Err(誤り) = バッファ.零で埋める(転送係.転送環境(), 露出状態のバイト数) {
        バッファ.破棄する(device);
        return Err(誤り);
    }
    Ok(バッファ)
}
