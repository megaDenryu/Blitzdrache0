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
    /// ポストプロセス有効時のみ`Some`(判断41)。ブルームピラミッドの全段。
    pub(crate) ブルーム: Option<ブルーム画像>,
}

/// ブルームピラミッドの中間画像(判断41)。縮小一覧[0]が1/2解像度で以降1/2ずつ小さくなり、
/// 拡大一覧[i]は縮小一覧[i]と同解像度(長さは縮小一覧の長さ-1)。
pub(crate) struct ブルーム画像 {
    pub(crate) 縮小一覧: Vec<(vk::Image, vk::ImageView)>,
    pub(crate) 拡大一覧: Vec<(vk::Image, vk::ImageView)>,
    pub(crate) 寸法一覧: Vec<vk::Extent2D>,
}
