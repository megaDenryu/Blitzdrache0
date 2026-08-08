//! 1フレームの描画先・参照画像の束。record層への引数肥大化を防ぐ(判断38で導入)。

use ash::vk;

pub(crate) struct フレーム画像一式 {
    pub(crate) スワップチェーン画像: vk::Image,
    pub(crate) スワップチェーンビュー: vk::ImageView,
    pub(crate) 深度画像: vk::Image,
    pub(crate) 深度ビュー: vk::ImageView,
    pub(crate) シャドウマップ画像: vk::Image,
    /// シャドウマップ資源の一辺。グラフの登録寸法とシャドウパスのビューポートがこの1つの値を読む。
    pub(crate) シャドウマップ一辺: crate::cascade::影の一辺解像度,
    /// 全層を1つの2D配列として見るビュー。グラフへの登録に使う(アタッチメントには使わない)。
    pub(crate) シャドウマップ配列ビュー: vk::ImageView,
    /// 距離区分ごとに1層だけを見るビュー。距離区分別のシャドウ記録がアタッチメントとして使う。
    pub(crate) シャドウマップ距離区分別のビュー一覧: [vk::ImageView; crate::cascade::距離区分数],
    /// シーン描画と空のパスが第2のカラー添付として書く動きベクトル画像。時間再構成方式に依らず毎フレーム書くため`Option`にしない。
    pub(crate) 動きベクトル画像: vk::Image,
    pub(crate) 動きベクトルビュー: vk::ImageView,
    /// ポストプロセス有効時のみ`Some`(判断38)。シーン・粒子の描画先になり、明るさの圧縮パスが読む。
    pub(crate) hdr: Option<(vk::Image, vk::ImageView)>,
    /// ポストプロセス有効時のみ`Some`(判断41)。光のにじみピラミッドの全段。
    pub(crate) 光のにじみ: Option<光のにじみ画像>,
}

/// 光のにじみピラミッドの中間画像(判断41)。縮小一覧[0]が1/2解像度で以降1/2ずつ小さくなり、
/// 拡大一覧[i]は縮小一覧[i]と同解像度(長さは縮小一覧の長さ-1)。
pub(crate) struct 光のにじみ画像 {
    pub(crate) 縮小一覧: Vec<(vk::Image, vk::ImageView)>,
    pub(crate) 拡大一覧: Vec<(vk::Image, vk::ImageView)>,
    pub(crate) 寸法一覧: Vec<vk::Extent2D>,
}
