//! フレームシェーダー定数バッファ3本の生成局面。呼ばれるのはレンダラー生成時の1回だけであり、
//! 以降のフレームは書き込みと参照しかしない。途中で失敗したら、それまでに確保した器をその場で逆順に破棄する。
//! 部分的に生成された一式は呼び出し元から見えない。

use ash::vk;

use super::buffer_set::定数バッファ一式;
use super::{cascade_bytes, sky_bytes, view_pass_bytes, フレームシェーダー定数一式};
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
) -> Result<フレームシェーダー定数一式, レンダラーエラー> {
    let ビューとシーンパス = 定数バッファ一式::生成する(device, メモリプロパティ, view_pass_bytes::バイト長)?;
    let 多段影 = match 定数バッファ一式::生成する(device, メモリプロパティ, cascade_bytes::バイト長) {
        Ok(値) => 値,
        Err(誤り) => {
            ビューとシーンパス.破棄する(device);
            return Err(誤り);
        }
    };
    let 空パス = match 定数バッファ一式::生成する(device, メモリプロパティ, sky_bytes::バイト長) {
        Ok(値) => 値,
        Err(誤り) => {
            多段影.破棄する(device);
            ビューとシーンパス.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(フレームシェーダー定数一式 {
        ビューとシーンパス,
        多段影,
        空パス,
    })
}
