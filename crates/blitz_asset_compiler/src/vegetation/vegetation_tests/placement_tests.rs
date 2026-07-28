//! 配置生成の検査。同じ個体数から同じ配置列を得ること、個体数で配置列が変わること、
//! 受け取れない個体数を拒むこと、生成した配置がチャンクの内側に収まることを見る。

use crate::vegetation::placement;

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
