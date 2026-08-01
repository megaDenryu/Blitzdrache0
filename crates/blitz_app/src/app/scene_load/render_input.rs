//! engineの描画対象一覧をrenderの非空な描画シーン素材へ変換し、同じ走査で可視判定の材料とプリミティブ描画項目も作る。
//! 3つを同じ走査で作るのは、可視個体選択もプリミティブ描画項目の可視個体区間参照も束の中での描画対象添字で引かれ、その添字がここで決まる並び順そのものだからである。
//! 走査順と配置の複製だけをここが決め、描画対象1つぶんを3つへ写す工程は`tray`が持つ。

mod anchor;
#[cfg(test)]
mod anchor_tests;
mod error;
mod instance_transforms;
mod layout;
#[cfg(test)]
mod layout_tests;
mod material_slots;
#[cfg(test)]
mod reader_tests;
mod tray;
#[cfg(test)]
mod tray_fixture;
#[cfg(test)]
mod tray_tests;
mod visibility_material;

use blitz_engine::primitive_draw_item::プリミティブ描画項目一覧;
use blitz_engine::{シーンデータ, チャンク座標};
use blitz_math::大域ワールド位置;
use blitz_render::描画シーン素材;

pub(crate) use error::描画入力エラー;

use super::convert;
use crate::app::visibility::群可視材料の登録;
use crate::error::起動エラー;

/// 1つの束へ載せる描画入力の一式。描画シーン素材はレンダラーが、登録一式はアプリが持つ2つの台帳が受け取る。
pub(crate) struct 束の描画入力 {
    pub(crate) 描画シーン: 描画シーン素材,
    pub(crate) 登録一式: 束の登録一式,
}

/// レンダラーへ描画シーンを渡した後に、同じ束IDでアプリの台帳へ入れる2つ。束の解除では同じ束IDで2つとも消える。
pub(crate) struct 束の登録一式 {
    pub(crate) 可視材料一覧: Vec<群可視材料の登録>,
    pub(crate) プリミティブ描画項目一覧: プリミティブ描画項目一覧,
}

pub(super) fn 変換する(
    シーン: &シーンデータ,
    並べ方: crate::cli::描画対象の並べ方,
    束座標: チャンク座標,
    大域平行移動: 大域ワールド位置,
) -> Result<束の描画入力, 起動エラー> {
    let 大域の基準原点 = anchor::導出する(シーン, 束座標, 大域平行移動)?;
    let 件数 = 並べ方.件数.map_or(シーン.描画対象一覧().len(), crate::cli::描画対象数::usize値);
    let mut 受け皿 = tray::変換の受け皿::生成する(件数);
    // 位置は束の中での並び順、添字はシーンデータの中での並び順である。走査順が逆順のときだけ2つが食い違い、
    // 描画対象とその配置変換の対応は変わらないまま、束の中の並びだけが逆になる。
    for 位置 in 0..件数 {
        let 添字 = 並べ方.走査順.シーン内の添字へ写す(位置, 件数);
        let 元 = &シーン.描画対象一覧()[添字 % シーン.描画対象一覧().len()];
        let 変換 = 並べ方.件数.map_or(元.ローカルからワールド(), |_| {
            元.ローカルからワールド().合成する(layout::配置する(添字, 件数))
        });
        受け皿.積む(元, 位置, 大域の基準原点, 変換)?;
    }
    Ok(受け皿.仕上げる())
}
