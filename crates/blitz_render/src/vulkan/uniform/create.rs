//! フレームシェーダー定数バッファ3本の生成局面。呼ばれるのはレンダラー生成時の1回だけであり、
//! 以降のフレームは書き込みと参照しかしない。途中で失敗したら、それまでに確保した器をその場で逆順に破棄する。
//! 部分的に生成された一式は呼び出し元から見えない。

use super::buffer_set::定数バッファ一式;
use super::{cascade_bytes, sky_bytes, view_bytes, フレームシェーダー定数一式};
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>) -> Result<フレームシェーダー定数一式, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let ビュー = 定数バッファ一式::生成する(確保係, view_bytes::バイト長)?;
    let 多段影 = match 定数バッファ一式::生成する(確保係, cascade_bytes::バイト長) {
        Ok(値) => 値,
        Err(誤り) => {
            ビュー.破棄する(device);
            return Err(誤り);
        }
    };
    let 空パス = match 定数バッファ一式::生成する(確保係, sky_bytes::バイト長) {
        Ok(値) => 値,
        Err(誤り) => {
            多段影.破棄する(device);
            ビュー.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(フレームシェーダー定数一式 {
        ビュー, 多段影, 空パス
    })
}
