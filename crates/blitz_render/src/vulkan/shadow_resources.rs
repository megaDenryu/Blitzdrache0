//! 影の2つの資源の組。方向光の多段影と点光源の影の立方体配列を1つの値として持ち、確保と破棄を対にする。
//!
//! 2つを1つの型にするのは、寿命がそろっているためである。どちらも生成時に1度だけ確保し、
//! スワップチェーンの再構築で作り直さない。別々のフィールドとして持ち回ると、確保の失敗で逆順に破棄する梯子と、
//! 資源を運ぶ経路の両方が資源の数だけ増える。

use super::point_light_shadow_map::点光源の影の立方体配列;
use super::shadow_map::シャドウマップ;
use super::tracked_device::GPUデバイス;
use crate::cascade::影の一辺解像度;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

/// 生成時に1度だけ確保する影の資源の組。
pub(crate) struct 影の資源の組 {
    pub(crate) 多段影: シャドウマップ,
    pub(crate) 点光源の影: 点光源の影の立方体配列,
}

impl 影の資源の組 {
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.点光源の影.破棄する(device);
        self.多段影.破棄する(device);
    }
}

pub(crate) fn 生成する(
    確保係: &GPU資源の確保係<'_>, 影の一辺: 影の一辺解像度
) -> Result<影の資源の組, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 多段影 = シャドウマップ::生成する(確保係, 影の一辺)?;
    match 点光源の影の立方体配列::生成する(確保係) {
        Ok(点光源の影) => Ok(影の資源の組 { 多段影, 点光源の影 }),
        Err(誤り) => {
            多段影.破棄する(device);
            Err(誤り)
        }
    }
}
