//! 遠方環境の消費の照合報告。注入した解析入力から板ごとの期待放射輝度を組み、その板が写った画素の実測と
//! 並べて1行ずつ出す。判定は`cargo xtask indirect-probe`がこの出力を読んで行う。
//!
//! 期待値の式をここで書き直さず`blitz_render`の消費式の正本へ委ねるのは、シェーダーがその正本の写しだからである。
//! この報告が正本の本番外の呼び出し元であり、写しが正本と一致していることを実機の絵で確かめる唯一の経路である。
//!
//! ポスト処理を外した実行を前提にする。スワップチェーンの形式がsRGBであるため、その構成の書き込みは
//! 「線形の放射輝度を0から1へ切り詰めてsRGB符号化した値」そのものになり、期待値を閉じた式で作れる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」

mod condition;
mod expected_color;
mod injection_dispatch;
mod plates;
mod projection;
mod report_error;
mod row;
mod specular_lookup;

use blitz_math::大域ワールド位置;
use blitz_render::distant_environment::derived::鏡面畳込みの解像度;
use blitz_render::読み戻し画像;

pub(crate) use condition::遠方環境の検収条件;

use crate::app::frame::フレーム視点;

/// 照合に要る材料。1フレームぶんの視点と読み戻し画像、それに注入した条件と太陽方向を束ねる。
pub(crate) struct 照合の材料<'a> {
    pub(crate) 条件: 遠方環境の検収条件,
    pub(crate) 画像: &'a 読み戻し画像,
    pub(crate) 視点情報: &'a フレーム視点,
    pub(crate) 大域ずらし量: 大域ワールド位置,
    /// 太陽から見た向き。ワールドの向きを太陽相対座標へ写すフレームがこの向きから決まる。
    pub(crate) 太陽方向: [f32; 3],
}

pub(crate) fn 代表板を照合する(材料: &照合の材料<'_>) {
    let 解析入力 = 材料.条件.焼き上がりの解析入力();
    let 期待の材料 = expected_color::期待の材料 {
        解析入力: &解析入力,
        鏡面の解像度: 鏡面畳込みの解像度::既定値(),
        太陽方向: 材料.太陽方向.map(f64::from),
    };
    for (番号, 板) in plates::一覧().iter().enumerate() {
        match 板1枚を照合する(材料, &期待の材料, 板, 番号) {
            Ok(()) => {}
            Err(理由) => println!("間接照明代表板 照合できなかった 番号={} 理由={理由}", 番号 + 1),
        }
    }
}

fn 板1枚を照合する(
    材料: &照合の材料<'_>,
    期待の材料: &expected_color::期待の材料<'_>,
    板: &plates::板の宣言,
    番号: usize,
) -> Result<(), report_error::代表板の照合エラー> {
    let 投影 = projection::板の中心を投影する(材料.視点情報, 板.中心, 材料.大域ずらし量, 材料.画像.幅(), 材料.画像.高さ())?;
    let 期待 = expected_color::期待を求める(期待の材料, 板, 投影.視線);
    let 実測 = 材料
        .画像
        .ピクセル(投影.横, 投影.縦)
        .ok_or(report_error::代表板の照合エラー::画素が画像の外にある {
            横: 投影.横, 縦: 投影.縦
        })?;
    row::行を出す(番号 + 1, &投影, &期待, [実測[0], 実測[1], 実測[2]]);
    Ok(())
}
