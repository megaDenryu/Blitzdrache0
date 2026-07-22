//! フレーム時間統計の百分位値と突発遅延件数を検証する。

use super::集計する;

#[test]
fn 四標本の統計を集計する() {
    let Some(統計) = 集計する(&[10.0, 40.0, 20.0, 30.0]) else {
        panic!("標本があるため統計を返すはず");
    };
    assert_eq!(統計.標本数, 4);
    assert_eq!(統計.平均ms, 25.0);
    assert_eq!(統計.p50ms, 20.0);
    assert_eq!(統計.p95ms, 40.0);
    assert_eq!(統計.p99ms, 40.0);
    assert_eq!(統計.最大ms, 40.0);
    assert_eq!(統計.二十五ms超過数, 2);
}

#[test]
fn 標本なしは統計を返さない() {
    assert!(集計する(&[]).is_none());
}
