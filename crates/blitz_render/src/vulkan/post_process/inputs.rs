//! ポスト処理一式が持つハンドルを、フレーム記録が読む描画入力と画像の組へ平坦化する。
//! 呼び出しタイミング: 毎フレームの描画入力組み立て時(生成完了後・破棄前であることは保持側の`レンダラー`が保証する)。

use ash::vk;

use super::ポスト処理一式;
use crate::vulkan::auto_exposure::自動露出描画入力;
use crate::vulkan::frame::{光のにじみ描画入力, 光のにじみ画像, 明るさの圧縮描画入力};

/// 光のにじみと明るさの圧縮の1フレーム入力。片方だけが存在する状態を作らせないため対で返す。
pub(crate) struct ポスト描画入力 {
    pub(crate) 光のにじみ: 光のにじみ描画入力,
    pub(crate) 明るさの圧縮: 明るさの圧縮描画入力,
    /// 露出方式がヒストグラム自動の世界だけ`Some`。時刻別固定の世界ではパスを1本も積まない。
    pub(crate) 自動露出: Option<自動露出描画入力>,
}

impl ポスト処理一式 {
    pub(crate) fn 描画入力を作る(&self, 露出: f32, 自動露出の経過秒: f32) -> ポスト描画入力 {
        ポスト描画入力 {
            光のにじみ: 光のにじみ描画入力 {
                前処理pipeline: self.光のにじみ.前処理pipeline,
                前処理layout: self.光のにじみ.前処理layout,
                縮小pipeline: self.光のにじみ.縮小pipeline,
                縮小layout: self.光のにじみ.縮小layout,
                拡大pipeline: self.光のにじみ.拡大pipeline,
                拡大layout: self.光のにじみ.拡大layout,
                前処理set: self.光のにじみ.前処理set,
                縮小set一覧: self.光のにじみ.縮小set一覧.clone(),
                拡大set一覧: self.光のにじみ.拡大set一覧.clone(),
            },
            明るさの圧縮: 明るさの圧縮描画入力 {
                pipeline: self.明るさの圧縮.pipeline,
                layout: self.明るさの圧縮.layout,
                ディスクリプタセット: self.明るさの圧縮.descriptor_set,
                露出,
                芸術的バイアスの補正段: self.自動露出.方式().芸術的バイアスの補正段(),
                自動か: u32::from(self.自動露出.方式().パスを積むか()),
            },
            自動露出: self
                .自動露出
                .方式()
                .パスを積むか()
                .then(|| self.自動露出.描画入力を作る(自動露出の経過秒)),
        }
    }

    /// シーン・粒子パスの描画先であり、明るさの圧縮パスが読むHDR中間画像。
    pub(crate) fn hdr画像組(&self) -> (vk::Image, vk::ImageView) {
        (self.hdrターゲット.画像, self.hdrターゲット.画像ビュー)
    }

    pub(crate) fn 光のにじみ画像を作る(&self) -> 光のにじみ画像 {
        光のにじみ画像 {
            縮小一覧: 画像組一覧にする(&self.光のにじみピラミッド.縮小一覧),
            拡大一覧: 画像組一覧にする(&self.光のにじみピラミッド.拡大一覧),
            寸法一覧: self.光のにじみピラミッド.寸法一覧.clone(),
        }
    }
}

fn 画像組一覧にする(一覧: &[crate::vulkan::hdr_target::HDRターゲット]) -> Vec<(vk::Image, vk::ImageView)> {
    一覧.iter().map(|画像| (画像.画像, 画像.画像ビュー)).collect()
}
