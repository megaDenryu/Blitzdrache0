//! 大気LUT一式の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに作った資源をその場で逆順に破棄する。部分的に生成された一式は呼び出し元から見えない。

use ash::vk;

use super::{image, medium_uniform, pipeline, transmittance_descriptor, 大気LUT一式};
use crate::atmosphere::大気LUT解像度;
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    解像度: 大気LUT解像度,
    シェーダー: &大気LUTシェーダー一式,
) -> Result<大気LUT一式, レンダラーエラー> {
    let 透過率 = image::大気LUT画像::生成する(device, メモリプロパティ, 透過率の寸法(解像度))?;
    let 媒体ユニフォーム = match medium_uniform::媒体ユニフォーム一式::生成する(device, メモリプロパティ) {
        Ok(値) => 値,
        Err(誤り) => {
            透過率.破棄する(device);
            return Err(誤り);
        }
    };
    let ユニフォーム一覧 = ユニフォーム一覧を集める(&媒体ユニフォーム);
    let 透過率ディスクリプタ =
        match transmittance_descriptor::透過率ディスクリプタ::生成する(device, ユニフォーム一覧, 透過率.画像ビュー) {
            Ok(値) => 値,
            Err(誤り) => {
                媒体ユニフォーム.破棄する(device);
                透過率.破棄する(device);
                return Err(誤り);
            }
        };
    match pipeline::生成パイプライン::生成する(device, 透過率ディスクリプタ.layout, シェーダー.透過率.コード()) {
        Ok(透過率パイプライン) => Ok(大気LUT一式 {
            解像度,
            透過率,
            媒体ユニフォーム,
            透過率ディスクリプタ,
            透過率パイプライン,
        }),
        Err(誤り) => {
            透過率ディスクリプタ.破棄する(device);
            媒体ユニフォーム.破棄する(device);
            透過率.破棄する(device);
            Err(誤り)
        }
    }
}

fn 透過率の寸法(解像度: 大気LUT解像度) -> vk::Extent2D {
    vk::Extent2D {
        width: 解像度.透過率の幅(),
        height: 解像度.透過率の高さ(),
    }
}

fn ユニフォーム一覧を集める(
    ユニフォーム: &medium_uniform::媒体ユニフォーム一式
) -> [vk::Buffer; フレームインフライト数] {
    let mut 一覧 = [vk::Buffer::null(); フレームインフライト数];
    for 添字 in フレームスロット添字::全スロット() {
        一覧[添字.配列添字()] = ユニフォーム.buffer(添字);
    }
    一覧
}
