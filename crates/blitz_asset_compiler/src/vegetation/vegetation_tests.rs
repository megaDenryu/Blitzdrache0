//! 植生の配置生成が決定的であることと、焼いた群の構造が保つべき条件を満たすことの検査。

#![allow(clippy::unwrap_used)]

use blitz_engine::{メッシュデータ, メッシュ頂点属性, 個体配置};

use super::{bounds, placement};

#[test]
fn 同じ個体数からは同じ配置列を得る() {
    let 一回目 = placement::配置列を作る(64).unwrap();
    let 二回目 = placement::配置列を作る(64).unwrap();
    assert_eq!(一回目, 二回目);
    assert_eq!(一回目.len(), 64);
}

#[test]
fn 個体数が変わると配置列も変わる() {
    let 少ない = placement::配置列を作る(4).unwrap();
    let 多い = placement::配置列を作る(64).unwrap();
    assert_ne!(少ない.len(), 多い.len());
    assert_ne!(少ない[0], 多い[0]);
}

#[test]
fn 個体数0と上限超えを拒否する() {
    assert!(placement::配置列を作る(0).is_err());
    assert!(placement::配置列を作る(usize::MAX).is_err());
}

#[test]
fn 全ての個体がチャンクの内側に立つ() {
    for 配置 in placement::配置列を作る(64).unwrap() {
        let [x, y, z] = 配置.平行移動();
        assert!((0.0..=100.0).contains(&x), "X={x}がチャンクの外にある");
        assert!((0.0..=100.0).contains(&z), "Z={z}がチャンクの外にある");
        assert_eq!(y, 0.0);
    }
}

#[test]
fn 境界は全個体を覆う() {
    let 段一覧 = vec![箱を作る()];
    let 配置一覧 = placement::配置列を作る(16).unwrap();
    let 境界 = bounds::境界を求める(&段一覧, &配置一覧).unwrap();
    let 包囲 = 境界.群の包囲領域();
    for 配置 in &配置一覧 {
        let [x, _, z] = 配置.平行移動();
        assert!(包囲.最小()[0] <= x && x <= 包囲.最大()[0]);
        assert!(包囲.最小()[2] <= z && z <= 包囲.最大()[2]);
    }
    assert!(境界.原型の境界球().半径() > 0.0);
}

#[test]
fn 広がりを持たない原型を拒否する() {
    let 一点 = メッシュデータ {
        頂点一覧: vec![頂点を作る([0.0, 0.0, 0.0])],
        インデックス一覧: vec![0],
        スキン頂点属性一覧: None,
    };
    let 配置一覧 = vec![個体配置::生成する([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0; 3]).unwrap()];
    assert!(bounds::境界を求める(&[一点], &配置一覧).is_err());
}

fn 箱を作る() -> メッシュデータ {
    let 位置一覧 = [[-0.8, 0.0, -0.8], [0.8, 0.0, -0.8], [0.8, 3.0, 0.8], [-0.8, 3.0, 0.8]];
    メッシュデータ {
        頂点一覧: 位置一覧.into_iter().map(頂点を作る).collect(),
        インデックス一覧: vec![0, 1, 2, 0, 2, 3],
        スキン頂点属性一覧: None,
    }
}

fn 頂点を作る(位置: [f32; 3]) -> メッシュ頂点属性 {
    メッシュ頂点属性 {
        位置,
        法線: [0.0, 1.0, 0.0],
        接線: [1.0, 0.0, 0.0, 1.0],
        uv: [0.0, 0.0],
    }
}
