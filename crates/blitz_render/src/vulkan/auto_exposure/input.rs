//! 自動露出の資源を、フレーム記録が読む1フレーム入力へ平坦化する。
//! 呼び出しタイミング: 毎フレームの描画入力組み立て時(生成完了後・破棄前であることは保持側の`レンダラー`が保証する)。

use ash::vk;

use super::自動露出一式;

/// 集計と導出のパスが要るハンドルと即時定数。露出方式が時刻別固定の世界ではパスを積まないため使われない。
#[derive(Clone)]
pub(crate) struct 自動露出描画入力 {
    pub(crate) ヒストグラムバッファ: vk::Buffer,
    pub(crate) 露出状態バッファ: vk::Buffer,
    pub(crate) 集計pipeline: vk::Pipeline,
    pub(crate) 集計layout: vk::PipelineLayout,
    pub(crate) 導出pipeline: vk::Pipeline,
    pub(crate) 導出layout: vk::PipelineLayout,
    /// 集計と導出が共有する1つのセット。どちらがどの束縛を触るかはレンダーグラフのパス宣言が持つ。
    pub(crate) セット: vk::DescriptorSet,
    /// 導出が押し込む定数の並び。順は`shaders/auto_exposure_resolve.slang`の`ResolveSetting`と一致させる。
    pub(crate) 導出の即時定数: [f32; 6],
}

impl 自動露出一式 {
    pub(crate) fn 描画入力を作る(&self) -> 自動露出描画入力 {
        自動露出描画入力 {
            ヒストグラムバッファ: self.バッファ.ヒストグラム.handle,
            露出状態バッファ: self.バッファ.露出状態.handle,
            集計pipeline: self.パイプライン.集計,
            集計layout: self.パイプライン.集計レイアウト,
            導出pipeline: self.パイプライン.導出,
            導出layout: self.パイプライン.導出レイアウト,
            セット: self.ディスクリプタ.セット,
            導出の即時定数: [
                self.設定.目標中間灰の対数輝度,
                self.設定.補正段の下限,
                self.設定.補正段の上限,
                self.設定.補正段を下げる速さ,
                self.設定.補正段を上げる速さ,
                self.設定.一フレームの経過秒,
            ],
        }
    }

    /// 世界が宣言した露出の決め方。パスを積むかと、明るさの圧縮へ渡す芸術的バイアスの段を決める。
    pub(crate) fn 方式(&self) -> crate::auto_exposure::露出方式 {
        self.設定.方式
    }
}
