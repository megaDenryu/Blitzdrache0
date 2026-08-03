//! 解析入力の注入資源の生成局面。呼ばれるのは検収が注入の入口を叩いた1回だけであり、以降のフレームは
//! 出来上がったバッファを転送元に取るだけである。途中で失敗したら、それまでに確保したバッファを逆順に破棄する。

use ash::vk;

use super::upload_buffer::注入元バッファ;
use super::{bytes, 解析入力の注入資源};
use crate::distant_environment::遠方環境の解析入力;
use crate::error::レンダラーエラー;
use crate::indirect_lighting::間接照明エラー;
use crate::vulkan::derived_environment::派生表現一式;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    派生表現: &派生表現一式,
    入力: &遠方環境の解析入力,
) -> Result<解析入力の注入資源, レンダラーエラー> {
    入力
        .粗さ段数と噛み合うか(派生表現.鏡面畳込みの解像度().粗さ段数())
        .map_err(|誤り| レンダラーエラー::from(間接照明エラー::from(誤り)))?;
    let 拡散照度 = 注入元バッファ::生成する(device, メモリプロパティ, &bytes::拡散照度のバイト列(入力, 派生表現.拡散照度の解像度()))?;
    let 鏡面畳込み = match 注入元バッファ::生成する(
        device,
        メモリプロパティ,
        &bytes::鏡面畳込みのバイト列(入力, 派生表現.鏡面畳込みの解像度()),
    ) {
        Ok(バッファ) => バッファ,
        Err(誤り) => {
            拡散照度.破棄する(device);
            return Err(誤り);
        }
    };
    match 注入元バッファ::生成する(
        device,
        メモリプロパティ,
        &bytes::反射率積分表のバイト列(入力, 派生表現.反射率積分表の解像度()),
    ) {
        Ok(反射率積分表) => Ok(解析入力の注入資源 {
            拡散照度,
            鏡面畳込み,
            反射率積分表,
        }),
        Err(誤り) => {
            鏡面畳込み.破棄する(device);
            拡散照度.破棄する(device);
            Err(誤り)
        }
    }
}
