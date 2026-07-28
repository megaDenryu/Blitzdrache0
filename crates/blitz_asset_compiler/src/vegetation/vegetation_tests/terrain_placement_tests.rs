//! 地形へ同居させる配置生成の検査。同じチャンクからは同じ配置列を得ること、チャンクが変わると並びも変わること、
//! 個体がチャンクの内側に収まること、平行移動のYが渡した地表高さそのものになることを見る。

use blitz_engine::チャンク座標;

use crate::error::アセットコンパイルエラー;
use crate::vegetation::terrain_placement;

/// 座標から一意に決まる検査用の地表高さ。配置のYがこの値をそのまま運ぶことを見るため、XとZの両方に依存させる。
fn 検査用の高さ(x: f32, z: f32) -> Result<f32, アセットコンパイルエラー> {
    Ok(x * 0.25 - z * 0.5)
}

#[test]
fn 同じチャンクと同じ個体数からは同じ配置列を得る() {
    let 座標 = チャンク座標::生成する(-2, 1);
    let 一回目 = terrain_placement::配置列を作る(座標, 64, 検査用の高さ).unwrap();
    let 二回目 = terrain_placement::配置列を作る(座標, 64, 検査用の高さ).unwrap();
    assert_eq!(一回目, 二回目);
    assert_eq!(一回目.len(), 64);
}

#[test]
fn チャンクが変わると配置列も変わる() {
    let 原点 = terrain_placement::配置列を作る(チャンク座標::生成する(0, 0), 64, 検査用の高さ).unwrap();
    let 隣 = terrain_placement::配置列を作る(チャンク座標::生成する(1, 0), 64, 検査用の高さ).unwrap();
    assert_eq!(原点.len(), 隣.len());
    assert_ne!(原点, 隣);
}

#[test]
fn 個体数0と上限超えを拒否する() {
    let 座標 = チャンク座標::生成する(0, 0);
    assert!(terrain_placement::配置列を作る(座標, 0, 検査用の高さ).is_err());
    assert!(terrain_placement::配置列を作る(座標, usize::MAX, 検査用の高さ).is_err());
}

#[test]
fn 全ての個体がチャンクの内側に立ち高さは地表に従う() {
    for 配置 in terrain_placement::配置列を作る(チャンク座標::生成する(2, -2), 64, 検査用の高さ).unwrap() {
        let [x, y, z] = 配置.平行移動();
        assert!((0.0..=100.0).contains(&x), "X={x}がチャンクの外にある");
        assert!((0.0..=100.0).contains(&z), "Z={z}がチャンクの外にある");
        assert_eq!(y, 検査用の高さ(x, z).unwrap());
    }
}

/// 地表高さの取得が失敗したら、その失敗を握り潰さずそのまま返すこと。
#[test]
fn 地表高さの失敗をそのまま返す() {
    let 結果 = terrain_placement::配置列を作る(チャンク座標::生成する(0, 0), 4, |_, _| {
        Err(アセットコンパイルエラー::地表高さの座標範囲外(0.0))
    });
    assert!(matches!(結果, Err(アセットコンパイルエラー::地表高さの座標範囲外(_))));
}
