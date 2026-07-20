//! GPUバイト列化の単体テスト(判断55): 件数×固定長とバイト列長が一致すること。

#![allow(clippy::unwrap_used)]

use crate::cloth::{布データ, 布仕様, 布を生成する};

use super::adjacency_bytes::隣接拘束バイト列にする;
use super::constraint_bytes::拘束バイト列にする;
use super::particle_bytes::粒子バイト列にする;

fn 試験用布() -> 布データ {
    let 仕様 = 布仕様::生成する(4, 1.0, 10.0, [0.0, 0.0, 0.0]).unwrap();
    布を生成する(&仕様).unwrap()
}

#[test]
fn 粒子バイト列長は件数times32になる() {
    let 布 = 試験用布();
    assert_eq!(粒子バイト列にする(&布).len(), 布.粒子一覧.len() * 32);
}

#[test]
fn 拘束バイト列長は件数times16になる() {
    let 布 = 試験用布();
    assert_eq!(拘束バイト列にする(&布).len(), 布.距離拘束一覧.len() * 16);
}

#[test]
fn 隣接拘束バイト列長は粒子数times64になる() {
    let 布 = 試験用布();
    assert_eq!(隣接拘束バイト列にする(&布).len(), 布.粒子一覧.len() * 64);
}
