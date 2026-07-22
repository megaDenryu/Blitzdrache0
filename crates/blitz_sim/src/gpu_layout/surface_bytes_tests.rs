#![allow(clippy::unwrap_used)]

use super::表面流バイト列にする;
use crate::{表面流仕様, 表面流状態};

#[test]
fn 表面流セルを行優先の32バイト表現へ変換する() {
    let 仕様 = 表面流仕様::生成する([2, 2], 1.0, 0.1, [0.0, -1.0], 0.0).unwrap();
    let 状態 = 表面流状態::液膜分布で生成する(&仕様, |[列, _]| if 列 == 0 { 0.25 } else { 0.5 }).unwrap();
    let バイト列 = 表面流バイト列にする(&状態);

    assert_eq!(バイト列.len(), 128);
    assert_eq!(&バイト列[0..4], &0.25f32.to_le_bytes());
    assert_eq!(&バイト列[32..36], &0.5f32.to_le_bytes());
}
