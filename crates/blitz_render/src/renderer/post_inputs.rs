//! スキニング・ブルーム・トーンマップの1フレーム入力の組み立て。
//! `draw_dispatch`の行数分割のためだけに切り出した内部ヘルパー。

use super::レンダラー;
use crate::vulkan::frame::{スキニング描画入力, トーンマップ描画入力, ブルーム描画入力};

pub(super) struct ポスト入力束 {
    pub(super) スキニング: Option<スキニング描画入力>,
    pub(super) ブルーム: Option<ブルーム描画入力>,
    pub(super) トーンマップ: Option<トーンマップ描画入力>,
}

impl レンダラー {
    pub(super) fn ポスト入力束を組み立てる(&self, フレーム添字: usize, 露出: f32) -> ポスト入力束 {
        let スキニング = self.スキニング.as_ref().map(|一式| スキニング描画入力 {
            pipeline: 一式.パイプラインhandle(),
            layout: 一式.パイプラインlayout(),
            ディスクリプタセット: 一式.set(フレーム添字),
            頂点数: 一式.頂点数,
            出力バッファ: 一式.出力バッファ(),
        });
        let ブルーム = self.ブルーム.as_ref().map(|一式| ブルーム描画入力 {
            前処理pipeline: 一式.前処理pipeline,
            前処理layout: 一式.前処理layout,
            縮小pipeline: 一式.縮小pipeline,
            縮小layout: 一式.縮小layout,
            拡大pipeline: 一式.拡大pipeline,
            拡大layout: 一式.拡大layout,
            前処理set: 一式.前処理set,
            縮小set一覧: 一式.縮小set一覧.clone(),
            拡大set一覧: 一式.拡大set一覧.clone(),
        });
        // ポストプロセスの入力と画像は対で生成される(判断38。graph_build側の不変条件)。
        let トーンマップ = self.トーンマップ.as_ref().map(|一式| トーンマップ描画入力 {
            pipeline: 一式.pipeline,
            layout: 一式.layout,
            ディスクリプタセット: 一式.descriptor_set,
            露出,
        });
        ポスト入力束 {
            スキニング,
            ブルーム,
            トーンマップ,
        }
    }
}
