//! 記録クロージャに渡す内部型。宣言済みハンドルの解決とコマンドバッファ取得だけを許す
//! （判断28: 宣言=真実。参照: `_doc/設計/レンダーグラフ.md`「パス」）。

use ash::vk;

use super::handle::画像ハンドル;
use super::registry::画像レジストリ;

pub(crate) struct 記録文脈<'a> {
    device: &'a ash::Device,
    command_buffer: vk::CommandBuffer,
    レジストリ: &'a 画像レジストリ,
    パス名: &'static str,
    宣言済み画像: Vec<画像ハンドル>,
}

impl<'a> 記録文脈<'a> {
    pub(crate) fn 生成する(
        device: &'a ash::Device,
        command_buffer: vk::CommandBuffer,
        レジストリ: &'a 画像レジストリ,
        パス名: &'static str,
        宣言済み画像: Vec<画像ハンドル>,
    ) -> Self {
        Self { device, command_buffer, レジストリ, パス名, 宣言済み画像 }
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
        self.レジストリ.画像を取得する(ハンドル)
    }
}
