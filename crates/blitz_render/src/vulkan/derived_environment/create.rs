//! 派生表現一式の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけであり、
//! 以降のフレームは束縛を参照するだけである。画像・ディスクリプタ・パイプラインの3つの組を順に作り、
//! 途中で失敗したらそれまでに作った組をその場で逆順に片付ける。
//!
//! 受け取るのは3つの解像度と遠方環境の2次元配列ビューと3本のシェーダー、返すのは組み上がった一式である。

use ash::vk;

use super::descriptors::派生表現のディスクリプタ三点;
use super::images::派生表現の画像三点;
use super::pipelines::{派生表現のシェーダー一式, 派生表現のパイプライン三点};
use super::派生表現一式;
use crate::distant_environment::derived::{反射率積分表の解像度, 拡散照度の解像度, 鏡面畳込みの解像度};
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

/// 3つの派生表現の解像度。引数の列が伸び続けるのを避けて1つに束ねる。
#[derive(Debug, Clone, Copy)]
pub(in crate::vulkan) struct 派生表現の解像度一式 {
    pub(in crate::vulkan) 拡散照度: 拡散照度の解像度,
    pub(in crate::vulkan) 鏡面畳込み: 鏡面畳込みの解像度,
    pub(in crate::vulkan) 反射率積分表: 反射率積分表の解像度,
}

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    解像度: 派生表現の解像度一式,
    遠方環境の配列ビュー: vk::ImageView,
    シェーダー: &派生表現のシェーダー一式<'_>,
) -> Result<派生表現一式, レンダラーエラー> {
    let 画像 = 派生表現の画像三点::生成する(device, メモリプロパティ, 解像度)?;
    let ディスクリプタ = match 派生表現のディスクリプタ三点::生成する(device, 遠方環境の配列ビュー, &画像) {
        Ok(ディスクリプタ) => ディスクリプタ,
        Err(誤り) => {
            画像.破棄する(device);
            return Err(誤り);
        }
    };
    match 派生表現のパイプライン三点::生成する(device, &ディスクリプタ, シェーダー) {
        Ok(パイプライン) => Ok(派生表現一式 {
            画像,
            ディスクリプタ,
            パイプライン,
            解像度,
        }),
        Err(誤り) => {
            ディスクリプタ.破棄する(device);
            画像.破棄する(device);
            Err(誤り)
        }
    }
}
