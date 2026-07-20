//! 1フレームの描画先・参照画像の束。record層への引数肥大化を防ぐ(判断38で導入)。

use ash::vk;

pub(crate) struct フレーム画像一式 {
    pub(crate) スワップチェーン画像: vk::Image,
    pub(crate) スワップチェーンビュー: vk::ImageView,
    pub(crate) 深度画像: vk::Image,
    pub(crate) 深度ビュー: vk::ImageView,
    pub(crate) シャドウマップ画像: vk::Image,
    pub(crate) シャドウマップビュー: vk::ImageView,
    /// ポストプロセス有効時のみ`Some`(判断38)。シーン・粒子の描画先になり、トーンマップパスが読む。
    pub(crate) hdr: Option<(vk::Image, vk::ImageView)>,
    /// ポストプロセス有効時のみ`Some`(判断39)。1/2解像度のピンポン2枚。
    pub(crate) ブルーム: Option<ブルーム画像>,
}

/// ブルームの中間画像(判断39)。aは抽出結果と縦ぼかし後の最終結果、bは横ぼかしの中間結果。
#[derive(Clone, Copy)]
pub(crate) struct ブルーム画像 {
    pub(crate) a画像: vk::Image,
    pub(crate) aビュー: vk::ImageView,
    pub(crate) b画像: vk::Image,
    pub(crate) bビュー: vk::ImageView,
    pub(crate) 寸法: vk::Extent2D,
}
