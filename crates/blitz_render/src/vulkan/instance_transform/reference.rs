//! ジオメトリのセットのbinding6へ結ぶ個体レコードのバッファとバイト範囲。フレームスロットごとに引くのは、
//! 動く個体を持つ描画対象がスロットごとに別のバッファを持つためである。
//! 読込時に一度だけ書く静的な対象は全スロットで同じバッファを指すため、束縛の書き込み側に枝が現れない。

use ash::vk;

use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

#[derive(Clone, Copy)]
pub(crate) struct 個体レコード参照 {
    スロット別バッファ: [vk::Buffer; 進行中フレーム数],
    範囲: vk::DeviceSize,
}

impl 個体レコード参照 {
    /// 読込時に一度だけ書く1本のバッファを全スロットが指す形。
    pub(crate) fn 全スロット共通のバッファから生成する(buffer: vk::Buffer, 範囲: vk::DeviceSize) -> Self {
        Self {
            スロット別バッファ: [buffer; 進行中フレーム数],
            範囲,
        }
    }

    pub(crate) fn スロット別バッファから生成する(
        スロット別バッファ: [vk::Buffer; 進行中フレーム数], 範囲: vk::DeviceSize
    ) -> Self {
        Self {
            スロット別バッファ, 範囲
        }
    }

    pub(crate) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.スロット別バッファ[フレーム添字.配列添字()]
    }

    pub(crate) fn 範囲(&self) -> vk::DeviceSize {
        self.範囲
    }
}
