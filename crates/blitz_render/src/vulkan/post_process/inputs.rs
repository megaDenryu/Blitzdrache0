//! ポスト処理一式が持つハンドルを、フレーム記録が読む描画入力と画像の組へ平坦化する。
//! 呼び出しタイミング: 毎フレームの描画入力組み立て時(生成完了後・破棄前であることは保持側の`レンダラー`が保証する)。

use ash::vk;

use super::ポスト処理一式;
use crate::vulkan::frame::{トーンマップ描画入力, ブルーム描画入力, ブルーム画像};

/// ブルームとトーンマップの1フレーム入力。片方だけが存在する状態を作らせないため対で返す。
pub(crate) struct ポスト描画入力 {
    pub(crate) ブルーム: ブルーム描画入力,
    pub(crate) トーンマップ: トーンマップ描画入力,
}

impl ポスト処理一式 {
    pub(crate) fn 描画入力を作る(&self, 露出: f32) -> ポスト描画入力 {
        ポスト描画入力 {
            ブルーム: ブルーム描画入力 {
                前処理pipeline: self.ブルーム.前処理pipeline,
                前処理layout: self.ブルーム.前処理layout,
                縮小pipeline: self.ブルーム.縮小pipeline,
                縮小layout: self.ブルーム.縮小layout,
                拡大pipeline: self.ブルーム.拡大pipeline,
                拡大layout: self.ブルーム.拡大layout,
                前処理set: self.ブルーム.前処理set,
                縮小set一覧: self.ブルーム.縮小set一覧.clone(),
                拡大set一覧: self.ブルーム.拡大set一覧.clone(),
            },
            トーンマップ: トーンマップ描画入力 {
                pipeline: self.トーンマップ.pipeline,
                layout: self.トーンマップ.layout,
                ディスクリプタセット: self.トーンマップ.descriptor_set,
                露出,
            },
        }
    }

    /// シーン・粒子パスの描画先であり、トーンマップパスが読むHDR中間画像。
    pub(crate) fn hdr画像組(&self) -> (vk::Image, vk::ImageView) {
        (self.hdrターゲット.画像, self.hdrターゲット.画像ビュー)
    }

    pub(crate) fn ブルーム画像を作る(&self) -> ブルーム画像 {
        ブルーム画像 {
            縮小一覧: 画像組一覧にする(&self.ブルームピラミッド.縮小一覧),
            拡大一覧: 画像組一覧にする(&self.ブルームピラミッド.拡大一覧),
            寸法一覧: self.ブルームピラミッド.寸法一覧.clone(),
        }
    }
}

fn 画像組一覧にする(一覧: &[crate::vulkan::hdr_target::HDRターゲット]) -> Vec<(vk::Image, vk::ImageView)> {
    一覧.iter().map(|画像| (画像.画像, 画像.画像ビュー)).collect()
}
