//! 1フレームの描画先・参照画像の束。record層への引数肥大化を防ぐ(判断38で導入)。

use ash::vk;

/// 1フレームの描画先と参照先の画像をまとめた束。
///
/// 影のビューは2種類ずつ持つ。`シャドウマップ配列ビュー`と`点光源の影の立方体配列ビュー`は全層をまとめて見るビューであり、
/// グラフへの登録にだけ使う(アタッチメントには使わない)。`シャドウマップ距離区分別のビュー一覧`と`点光源の影の層別のビュー一覧`は
/// 1層だけを見るビューであり、距離区分別のシャドウ記録と面ごとの記録がアタッチメントとして使う(添字は距離区分の番号と層番号である)。
/// `動きベクトル画像`はシーン描画と空のパスが第2のカラー添付として書く。時間再構成方式に依らず毎フレーム書くため`Option`にしない。
/// `hdr`と`光のにじみ`はポストプロセス有効時のみ`Some`である(判断38・判断41)。
pub(crate) struct フレーム画像一式<'a> {
    pub(crate) スワップチェーン画像: vk::Image,
    pub(crate) スワップチェーンビュー: vk::ImageView,
    pub(crate) 深度画像: vk::Image,
    pub(crate) 深度ビュー: vk::ImageView,
    pub(crate) シャドウマップ画像: vk::Image,
    pub(crate) シャドウマップ一辺: crate::cascade::影の一辺解像度, // グラフの登録寸法とシャドウパスのビューポートが読む
    pub(crate) シャドウマップ配列ビュー: vk::ImageView,
    pub(crate) シャドウマップ距離区分別のビュー一覧: [vk::ImageView; crate::cascade::距離区分数],
    pub(crate) 点光源の影の画像: vk::Image,
    pub(crate) 点光源の影の立方体配列ビュー: vk::ImageView,
    pub(crate) 点光源の影の層別のビュー一覧: &'a [vk::ImageView],
    pub(crate) 動きベクトル画像: vk::Image,
    pub(crate) 動きベクトルビュー: vk::ImageView,
    pub(crate) hdr: Option<(vk::Image, vk::ImageView)>, // シーン・粒子の描画先。明るさの圧縮パスが読む
    pub(crate) 光のにじみ: Option<光のにじみ画像>,      // 光のにじみピラミッドの全段
}

/// 光のにじみピラミッドの中間画像(判断41)。縮小一覧[0]が1/2解像度で以降1/2ずつ小さくなり、
/// 拡大一覧[i]は縮小一覧[i]と同解像度(長さは縮小一覧の長さ-1)。
pub(crate) struct 光のにじみ画像 {
    pub(crate) 縮小一覧: Vec<(vk::Image, vk::ImageView)>,
    pub(crate) 拡大一覧: Vec<(vk::Image, vk::ImageView)>,
    pub(crate) 寸法一覧: Vec<vk::Extent2D>,
}
