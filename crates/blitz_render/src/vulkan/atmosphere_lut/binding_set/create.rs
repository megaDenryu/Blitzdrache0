//! 大気LUT束縛一式の生成局面。呼ばれるのは大気LUT一式の組み立て時の1回だけである。
//! ディスクリプタを`descriptors`が、パイプラインを`pipelines`が作り、ここは2段の順序と巻き戻しだけを持つ。
//! 部分的に生成された束は呼び出し元から見えない。

mod descriptors;
mod pipelines;

use super::大気LUT束縛一式;
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::atmosphere_lut::base_resources::大気LUT基盤資源;

pub(super) fn 生成する(
    device: &ash::Device,
    基盤: &大気LUT基盤資源,
    シェーダー: &大気LUTシェーダー一式,
) -> Result<大気LUT束縛一式, レンダラーエラー> {
    let ディスクリプタ = descriptors::作る(device, 基盤)?;
    match pipelines::作る(device, &ディスクリプタ, シェーダー) {
        Ok([透過率パイプライン, 多重散乱パイプライン, スカイビューパイプライン]) => Ok(大気LUT束縛一式 {
            透過率ディスクリプタ: ディスクリプタ.透過率,
            多重散乱ディスクリプタ: ディスクリプタ.多重散乱,
            スカイビューディスクリプタ: ディスクリプタ.スカイビュー,
            透過率パイプライン,
            多重散乱パイプライン,
            スカイビューパイプライン,
        }),
        Err(誤り) => {
            ディスクリプタ.破棄する(device);
            Err(誤り)
        }
    }
}
