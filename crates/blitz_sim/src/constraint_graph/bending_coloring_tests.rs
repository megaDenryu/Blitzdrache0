//! 曲げ拘束のバッチと彩色の検査: 同じ点を持つ拘束と範囲外の添字を拒むこと、色の区間が全拘束を覆い同じ色の中で点を共有しないこと。

#![allow(clippy::unwrap_used)]

use std::collections::BTreeSet;

use super::bending_batch::{曲げ拘束のバッチ, 添字付き曲げ拘束};
use super::bending_coloring::曲げ拘束の彩色;
use super::error::拘束グラフエラー;
use super::point_index::点添字;
use crate::xpbd::{曲げのコンプライアンス, 曲げ拘束の引数, 静止角};

fn 拘束(a: u32, b: u32, c: u32, d: u32) -> Result<添字付き曲げ拘束, 拘束グラフエラー> {
    添字付き曲げ拘束::生成する(
        点添字::生成する(a),
        点添字::生成する(b),
        点添字::生成する(c),
        点添字::生成する(d),
        曲げ拘束の引数::生成する(静止角::零(), 曲げのコンプライアンス::硬い()),
    )
}

#[test]
fn 同じ点を持つ拘束と範囲外の添字を拒む() {
    assert!(matches!(拘束(0, 1, 2, 1), Err(拘束グラフエラー::曲げ拘束が同じ点を持つ { .. })));
    assert!(matches!(
        曲げ拘束のバッチ::生成する(3, vec![拘束(0, 1, 2, 3).unwrap()]),
        Err(拘束グラフエラー::曲げ拘束の点添字が範囲外 { .. })
    ));
}

/// 4点を触る拘束は、辺を1本しか共有しない隣どうしでも同じ色になれない。
#[test]
fn 色の区間は全拘束を覆い同じ色の中で点を共有しない() {
    let 一覧 = vec![
        拘束(0, 1, 2, 3).unwrap(),
        拘束(1, 2, 0, 4).unwrap(),
        拘束(5, 6, 7, 8).unwrap(),
        拘束(3, 4, 1, 9).unwrap(),
    ];
    let 彩色 = 曲げ拘束の彩色::生成する(&曲げ拘束のバッチ::生成する(10, 一覧).unwrap());
    let 合計: u32 = 彩色.色の区間一覧().iter().map(|区間| 区間.本数).sum();
    assert_eq!(usize::try_from(合計).unwrap(), 彩色.拘束の数());
    assert_eq!(彩色.色の数(), 3);
    for 区間 in 彩色.色の区間一覧() {
        let 開始 = 区間.開始.配列添字();
        let mut 触った点 = BTreeSet::new();
        for 拘束 in &彩色.拘束一覧()[開始..開始 + usize::try_from(区間.本数).unwrap()] {
            assert!(拘束.点一覧().iter().all(|点| 触った点.insert(*点)));
        }
    }
}
