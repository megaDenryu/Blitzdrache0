//! 記録クロージャに渡す内部型。宣言済みハンドルの解決とコマンドバッファ取得だけを許す
//! （判断28: 宣言=真実。参照: `_doc/設計/レンダーグラフ.md`「パス」）。

use ash::vk;

use super::buffer_registry::バッファレジストリ;
use super::handle::{バッファハンドル, 画像ハンドル};
use super::registry::画像レジストリ;

pub(crate) struct 記録文脈<'a> {
    device: &'a ash::Device,
    command_buffer: vk::CommandBuffer,
    画像レジストリ: &'a 画像レジストリ,
    バッファレジストリ: &'a バッファレジストリ,
    パス名: &'static str,
    宣言済み画像: Vec<画像ハンドル>,
    宣言済みバッファ: Vec<バッファハンドル>,
}

impl<'a> 記録文脈<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn 生成する(
        device: &'a ash::Device,
        command_buffer: vk::CommandBuffer,
        画像レジストリ: &'a 画像レジストリ,
        バッファレジストリ: &'a バッファレジストリ,
        パス名: &'static str,
        宣言済み画像: Vec<画像ハンドル>,
        宣言済みバッファ: Vec<バッファハンドル>,
    ) -> Self {
        Self { device, command_buffer, 画像レジストリ, バッファレジストリ, パス名, 宣言済み画像, 宣言済みバッファ }
    }

    pub(crate) fn device(&self) -> &ash::Device {
        self.device
    }

    pub(crate) fn コマンドバッファ(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    /// 宣言済みの画像ハンドルをvk::Imageへ解決する。
    ///
    /// 注意: 宣言(読み/書き)に無いハンドルを渡すのはプログラムのバグ（パスが自分の
    /// 宣言していないリソースへ触れようとした）であり、ここでpanicして知らせる。
    pub(crate) fn 画像を解決する(&self, ハンドル: 画像ハンドル) -> vk::Image {
        if !self.宣言済み画像.contains(&ハンドル) {
            panic!("パス「{}」が宣言していない画像ハンドルを解決しようとした", self.パス名);
        }
        self.画像レジストリ.画像を取得する(ハンドル)
    }

    /// 宣言済みのバッファハンドルをvk::Bufferへ解決する。`画像を解決する`と同じ
    /// 「宣言=真実」規律で、未宣言のハンドルはpanicする。
    pub(crate) fn バッファを解決する(&self, ハンドル: バッファハンドル) -> vk::Buffer {
        if !self.宣言済みバッファ.contains(&ハンドル) {
            panic!("パス「{}」が宣言していないバッファハンドルを解決しようとした", self.パス名);
        }
        self.バッファレジストリ.バッファを取得する(ハンドル)
    }
}
