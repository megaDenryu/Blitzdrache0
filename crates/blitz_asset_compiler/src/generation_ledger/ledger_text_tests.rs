//! 生成台帳の本文の書式そのものの回帰検査。組み立てた綴りを固定の期待値と1文字ずつ突き合わせる。
//!
//! 組み立てと解析の自己往復では、書式と解析を同時に変えた退行を検出できない。台帳は決定性の突き合わせから
//! 除いてあるため、書式の不変を守るのはこの検査だけである。見出しの2つの内容ハッシュは既知のバイト列から
//! 求めた値であり、実行ファイルの中身にも実行の環境にも依らない。

#![allow(clippy::unwrap_used)]

use std::collections::BTreeMap;

use blitz_engine::チャンク座標;

use super::content_hash::内容ハッシュ;
use super::generator_version::生成器の版;
use super::heading::生成台帳の見出し;
use super::ledger_text::生成台帳の本文;
use super::map_seed::{マップ生成の乱数の種, 種の由来};

/// 挿入の順を東の順でも南の順でもない並びにして、綴りの並びが挿入の順でなく座標の順で決まることを確かめる。
/// 期待する並びは東(x)を先、南(z)を後に見る辞書式である。
const 挿入する順の座標一覧: [(i32, i32); 4] = [(1, -1), (-1, 2), (1, -2), (-1, -1)];

const 期待する台帳の綴り: &str = "blitz_generation_ledger 1\n\
seed 20260810\n\
generator_version 7\n\
generator_image dd041024dfeb38eb\n\
bake_options 4fb560225adbe2fe\n\
chunk -1 -1 4476609c79f9b72b\n\
chunk -1 2 4476609c79f9b72b\n\
chunk 1 -2 4476609c79f9b72b\n\
chunk 1 -1 4476609c79f9b72b\n";

#[test]
fn 台帳の綴りが固定の書式と一致する() {
    let 見出し = 生成台帳の見出し::読み取った値から組み立てる(
        種の由来::種から焼いた(マップ生成の乱数の種::生成する(20260810)),
        生成器の版::生成する(7),
        内容ハッシュ::バイト列から求める(b"generator image"),
        内容ハッシュ::バイト列から求める(b"bake options"),
    );
    let mut 記録 = BTreeMap::new();
    for (東, 南) in 挿入する順の座標一覧 {
        記録.insert(チャンク座標::生成する(東, 南), 内容ハッシュ::バイト列から求める(b"chunk source"));
    }
    assert_eq!(生成台帳の本文::組み立てる(見出し, &記録).綴り(), 期待する台帳の綴り);
}

/// 種を持たない台帳の綴りも固定する。欄を空にせず`none`と書くのは、種の欄が消えると行の位置が動くためである。
#[test]
fn 種を持たない台帳の綴りが固定の書式と一致する() {
    let 見出し = 生成台帳の見出し::読み取った値から組み立てる(
        種の由来::種を持たない,
        生成器の版::生成する(1),
        内容ハッシュ::バイト列から求める(b"generator image"),
        内容ハッシュ::バイト列から求める(b"bake options"),
    );
    let 綴り = 生成台帳の本文::組み立てる(見出し, &BTreeMap::new());
    assert_eq!(
        綴り.綴り(),
        "blitz_generation_ledger 1\nseed none\ngenerator_version 1\ngenerator_image dd041024dfeb38eb\nbake_options 4fb560225adbe2fe\n"
    );
}

/// 固定した綴りが今の解析でそのまま読めることも確かめる。書式だけを変えて解析を直し忘れる退行を止める。
#[test]
fn 固定の書式の綴りを今の解析が読める() {
    let 中身 = 生成台帳の本文::読み取った綴りから作る(期待する台帳の綴り.to_string()).解析する().unwrap();
    assert_eq!(中身.チャンクごとの内容ハッシュ.len(), 挿入する順の座標一覧.len());
    assert!(中身.チャンクごとの内容ハッシュ.contains_key(&チャンク座標::生成する(1, -2)));
}
