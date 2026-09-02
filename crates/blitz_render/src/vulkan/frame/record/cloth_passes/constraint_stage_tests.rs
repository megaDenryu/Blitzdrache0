//! 拘束の工程が零化→色ごと×反復回数の並びで積まれることの検証。材料は`test_fixtures`が持つ。

#![allow(clippy::unwrap_used)]

use super::test_fixtures::{刻み数を据えた入力, 検査の色の数, 積んだパス名一覧};
use crate::cloth_material::布の拘束の反復回数;

/// 反証: 零化を反復の中で積むと、反復ごとに乗数が零へ戻って正典式の累積が消える。零化は1刻みに1本だけ、拘束の前に積む。
/// 拘束のディスパッチは色の数×反復回数であり、同じ色の並びを反復のたびに繰り返す。
#[test]
fn 拘束の工程は零化一本の後に色の数かける反復回数だけ積まれる() {
    let 名前一覧 = 積んだパス名一覧(&刻み数を据えた入力(1));
    let 零化の位置 = 名前一覧.iter().position(|名前| *名前 == "布乗数零化").unwrap();
    let 最初の拘束の位置 = 名前一覧.iter().position(|名前| *名前 == "布拘束").unwrap();
    assert!(零化の位置 < 最初の拘束の位置);
    assert_eq!(名前一覧.iter().filter(|名前| **名前 == "布乗数零化").count(), 1);
    assert_eq!(
        名前一覧.iter().filter(|名前| **名前 == "布拘束").count(),
        検査の色の数 * usize::try_from(布の拘束の反復回数).unwrap()
    );
    assert!(名前一覧.len() <= usize::try_from(super::一刻みのパス数).unwrap() + 1);
}
