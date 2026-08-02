//! 世代番号の発行と公開の1対1の対応の検査。発行していない番号での公開と、同じ番号での2回目の公開を通さないこと、
//! 構築に失敗した後は次の番号を発行して公開できることを固定する。
//!
//! 通したときの害は、退役待ちと公開中に同じ番号の別内容が並び、材質GPU参照の世代一致検査が別の表を通すことである。

#![allow(clippy::unwrap_used)]

use crate::vulkan::material_table::generation_build::構築する;
use crate::vulkan::material_table::generation_id::資源表世代ID;
use crate::vulkan::material_table::ledger::資源表世代台帳;

use super::fixture::検査用供給元;
use super::material_fixture::余裕のあるレイアウト容量;

fn 世代を作る(
    供給元: &mut 検査用供給元, 世代id: 資源表世代ID
) -> crate::vulkan::material_table::generation::資源表世代<u32, ()> {
    構築する(供給元, 世代id, 余裕のあるレイアウト容量(), &[]).unwrap()
}

#[test]
#[should_panic(expected = "未消費の発行番号が無いまま")]
fn 発行していない世代の公開を通さない() {
    let mut 供給元 = 検査用供給元::常に成功する();
    let 初期世代 = 世代を作る(&mut 供給元, 資源表世代ID::最初());
    let 次の世代 = 世代を作る(&mut 供給元, 資源表世代ID::最初().次を作る().unwrap());
    let mut 台帳 = 資源表世代台帳::最初の世代を公開する(初期世代);
    台帳.公開する(次の世代);
}

#[test]
#[should_panic(expected = "未消費の発行番号が無いまま")]
fn 同じ番号の二重公開を通さない() {
    let mut 供給元 = 検査用供給元::常に成功する();
    let 初期世代 = 世代を作る(&mut 供給元, 資源表世代ID::最初());
    let mut 台帳 = 資源表世代台帳::最初の世代を公開する(初期世代);
    let 発行id = 台帳.次の世代idを発行する().unwrap();
    台帳.公開する(世代を作る(&mut 供給元, 発行id));
    台帳.公開する(世代を作る(&mut 供給元, 発行id));
}

#[test]
fn 構築に失敗した後は次の番号を発行して公開できる() {
    let mut 供給元 = 検査用供給元::常に成功する();
    let 初期世代 = 世代を作る(&mut 供給元, 資源表世代ID::最初());
    let mut 台帳 = 資源表世代台帳::最初の世代を公開する(初期世代);

    let 失敗した発行id = 台帳.次の世代idを発行する().unwrap();
    let mut 失敗する供給元 = 検査用供給元::指定回で失敗する(1);
    assert!(構築する(&mut 失敗する供給元, 失敗した発行id, 余裕のあるレイアウト容量(), &[]).is_err());

    let 次の発行id = 台帳.次の世代idを発行する().unwrap();
    assert_ne!(次の発行id, 失敗した発行id, "失敗した番号は再利用しない");
    台帳.公開する(世代を作る(&mut 供給元, 次の発行id));
    assert_eq!(台帳.公開中().世代id(), 次の発行id);
    assert_eq!(台帳.退役待ち件数(), 1);
}
