//! ポスト処理段階の描画入力: 光のにじみピラミッドと明るさの圧縮。どちらもフレーム構成にポスト処理段階があるときだけ資源になる。
//! 影・布・UIと同じく、サブシステムごとに入力の型を分けて持つ。

use ash::vk;

/// 光のにじみピラミッド(判断41)1フレームぶんの入力。ポストプロセス有効時のみ`Some`で渡す。
/// セット一覧の長さは段数-1(縮小set[i]は縮小[i+1]へのパスの読み元、拡大set[i]は拡大[i]へのパスの読み元)。
pub(crate) struct 光のにじみ描画入力 {
    pub(crate) 前処理pipeline: vk::Pipeline,
    pub(crate) 前処理layout: vk::PipelineLayout,
    pub(crate) 縮小pipeline: vk::Pipeline,
    pub(crate) 縮小layout: vk::PipelineLayout,
    pub(crate) 拡大pipeline: vk::Pipeline,
    pub(crate) 拡大layout: vk::PipelineLayout,
    pub(crate) 前処理set: vk::DescriptorSet,
    pub(crate) 縮小set一覧: Vec<vk::DescriptorSet>,
    pub(crate) 拡大set一覧: Vec<vk::DescriptorSet>,
}

/// 明るさの圧縮パス(判断38・39)1フレームぶんの入力。ポストプロセス有効時のみ`Some`で渡す。
pub(crate) struct 明るさの圧縮描画入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    /// 明るさの圧縮前にHDR輝度へ掛ける露出倍率(プッシュ定数で渡す)。時刻別固定の枝ではこれが最終倍率である。
    pub(crate) 露出: f32,
    /// ヒストグラム自動の枝だけが使う2つ。基準倍率へ足す芸術的バイアスの段と、その枝かどうか(1のときだけ画素段がGPU上の露出状態を読む)。
    pub(crate) 芸術的バイアスの補正段: f32,
    pub(crate) 自動か: u32,
}
