//! チャンクストリーミングの配線。CLIの起動設定から調停を組み立て、毎フレーム、カメラが確定した後のプレイヤー位置を渡して進める。
//! 判断も計算もエンジン側の`ストリーミング調停`が持ち、ここは組み立てと呼び出しと報告の有無だけを扱う。
//! 毎フレームの進行は`advance`にある。

mod advance;
mod build;
mod bundle_sync;
#[cfg(test)]
mod bundle_sync_tests;
mod directory_replace;
mod integration_route;
mod lod_override;
mod lod_sync;
mod measurement;
mod route;
#[cfg(test)]
mod route_tests;

use blitz_engine::{ストリーミングメモリ量, ストリーミング調停, チャンク一辺};
pub(in crate::app) use build::ストリーミング配線を構築する;
pub(crate) use measurement::ストリーミング要約;

pub(super) struct ストリーミング配線 {
    調停: ストリーミング調停,
    チャンク一辺: チャンク一辺,
    位置決定: route::プレイヤー位置決定,
    計測: measurement::ストリーミング計測,
    上限: ストリーミングメモリ量,
    報告する: bool,
    描画除外座標: Option<blitz_engine::チャンク座標>,
    /// 実破棄済み束IDの受け皿。毎フレーム使い回し、破棄バッチごとのヒープ再確保を避ける。
    実破棄受け皿: Vec<blitz_render::描画束ID>,
    /// 世界差し替えで旧台帳を消した後に届く実破棄を、新世界の台帳へ報告しないための識別一覧。
    差し替え前の実破棄待ち束: Vec<blitz_render::描画束ID>,
    地形lod: lod_sync::地形LOD配線,
}

impl ストリーミング配線 {
    pub(super) fn 一辺(&self) -> チャンク一辺 {
        self.チャンク一辺
    }

    pub(super) fn チャンク目録の一辺を検査する(
        &self,
        新しい目録: &blitz_engine::チャンク目録,
    ) -> Result<(), blitz_engine::チャンク目録差し替えエラー> {
        let 新一辺 = 新しい目録.一辺();
        if self.チャンク一辺 == 新一辺 {
            return Ok(());
        }
        Err(blitz_engine::チャンク目録差し替えエラー::チャンク一辺変更 {
            旧: self.チャンク一辺,
            新: 新一辺,
        })
    }

    /// 見送りフレーム数はレンダラーの計器であり配線は保持しないため、呼出し側が渡す。
    pub(super) fn 要約を作る(&self, 見送りフレーム数: u64) -> ストリーミング要約 {
        self.計測
            .要約を作る(self.調停.転送量(), self.上限, 見送りフレーム数, self.地形lod.段別の束数())
    }

    /// そのフレームで各描画束が描く詳細段の受け皿を、描画入力が借用している間だけ預ける。ここに無い束は最詳細段を描く。
    /// 描画は`&mut アプリ`を要るため、選択を`アプリ`の中から借りたままにはできない。
    pub(super) fn 地形詳細段選択を預かる(&mut self) -> Vec<blitz_render::地形詳細段選択> {
        self.地形lod.束別選択を預ける()
    }

    pub(super) fn 地形詳細段選択を返す(&mut self, 受け皿: Vec<blitz_render::地形詳細段選択>) {
        self.地形lod.束別選択を戻す(受け皿);
    }
}
