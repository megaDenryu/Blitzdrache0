//! 画像仮想リソースの世代付きハンドル。実体(vk::Image/vk::ImageView)は
//! グラフ実行器の画像レジストリだけが知り、宣言側は添字と世代の組でのみ参照する。

/// グラフに登録された画像を指す不透明な参照。
/// フィールドはprivateで、生成は`生成する`経由のみ許す（値オブジェクト様式）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct 画像ハンドル {
    添字: u32,
    世代: u32,
}

impl 画像ハンドル {
    pub(crate) fn 生成する(添字: u32, 世代: u32) -> Self {
        Self { 添字, 世代 }
    }

    pub(crate) fn 添字(&self) -> u32 {
        self.添字
    }

    pub(crate) fn 世代(&self) -> u32 {
        self.世代
    }
}
