//! 割り当て済みの材質のセットへ1つの資源表世代の現物を書き込む先。触れるのは1つのセットだけである。
//!
//! 呼び出しタイミング: 資源表世代を作るたびに1回である。レイアウトを決める局面と呼ばれる頻度が違う。

use ash::vk;

use super::{材質テクスチャ表の束縛番号, 材質レコードの束縛番号};
use crate::vulkan::descriptor::ディスクリプタの書き込み先;
use crate::vulkan::material_record::材質レコードバッファ;
use crate::vulkan::texture::テクスチャ;

pub(crate) struct 材質のセットの書き込み先<'書き込み>(ディスクリプタの書き込み先<'書き込み>);

impl<'書き込み> 材質のセットの書き込み先<'書き込み> {
    pub(crate) fn 生成する(device: &'書き込み ash::Device, セット: vk::DescriptorSet) -> Self {
        Self(ディスクリプタの書き込み先::生成する(device, セット))
    }

    /// 1つの資源表世代の材質レコード列と常駐画像をセットへ書き込む。書き込むのは常駐した枚数までであり、
    /// 残りの要素は部分束縛のまま未書込で残す。
    pub(crate) fn 世代の資源を結ぶ(&self, レコードバッファ: &材質レコードバッファ, 画像集合: &[テクスチャ]) {
        self.0.バッファの先頭からの範囲を結ぶ(
            材質レコードの束縛番号,
            vk::DescriptorType::STORAGE_BUFFER,
            レコードバッファ.バッファのハンドル(),
            レコードバッファ.範囲,
        );
        let ビュー一覧: Vec<vk::ImageView> = 画像集合.iter().map(|画像| 画像.image_view).collect();
        self.0
            .サンプラー無しの画像の並びを結ぶ(材質テクスチャ表の束縛番号, &ビュー一覧, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
    }
}
