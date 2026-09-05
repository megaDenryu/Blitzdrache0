//! 楽曲の検証のうち、パターンと曲構成に掛かる項目(名乗りの重複・格子の寸法・セルの値・節の指す先と繰り返し回数)を確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

#[test]
fn パターンの名乗りが重複していると拒む() {
    let mut 対象 = crate::common::楽曲の例();
    対象.パターン一覧.push(crate::common::最初のパターン());
    assert!(対象.検証する().is_err());
}

#[test]
fn 格子の本数がトラックの本数と違うと拒む() {
    let mut 対象 = crate::common::楽曲の例();
    対象.パターン一覧[0].格子.pop();
    assert!(対象.検証する().is_err());
}

#[test]
fn 格子の行数が音の並びの長さと違うと拒む() {
    let mut 対象 = crate::common::楽曲の例();
    対象.パターン一覧[0].格子[0] = crate::common::打点のない格子(2);
    assert!(対象.検証する().is_err());
}

#[test]
fn 行の長さが32でないと拒む() {
    let mut 対象 = crate::common::楽曲の例();
    対象.パターン一覧[0].格子[0].行一覧[0] = vec![0; 31];
    assert!(対象.検証する().is_err());
}

#[test]
fn セルの値は4まで受け入れ5を拒む() {
    let mut 通る = crate::common::楽曲の例();
    通る.パターン一覧[0].格子[0].行一覧[0][0] = 4;
    通る.検証する().unwrap();

    let mut 拒む = crate::common::楽曲の例();
    拒む.パターン一覧[0].格子[0].行一覧[0][0] = 5;
    assert!(拒む.検証する().is_err());
}

#[test]
fn 曲構成が実在しないパターンを指すと拒む() {
    let mut 対象 = crate::common::楽曲の例();
    対象.曲構成[0].パターンの名乗り = crate::common::パターンの名乗り("存在しないパターン");
    assert!(対象.検証する().is_err());
}

#[test]
fn 繰り返し回数は1から8だけを受け入れる() {
    for 値 in [1, 8] {
        let mut 対象 = crate::common::楽曲の例();
        対象.曲構成[0].繰り返し回数 = 値;
        対象.検証する().unwrap();
    }
    for 値 in [0, 9] {
        let mut 対象 = crate::common::楽曲の例();
        対象.曲構成[0].繰り返し回数 = 値;
        assert!(対象.検証する().is_err(), "繰り返し回数{値}を通してはならない");
    }
}

#[test]
fn パターンの表示名が空白だけなら拒む() {
    let mut 拒む = crate::common::楽曲の例();
    拒む.パターン一覧[0].表示名 = " ".to_string();
    assert!(拒む.検証する().is_err());

    let mut 通る = crate::common::楽曲の例();
    通る.パターン一覧[0].表示名 = "第1のパターン".to_string();
    通る.検証する().unwrap();
}

#[test]
fn 小節数は0と上限より大きい値を拒む() {
    let mut 下回る = crate::common::楽曲の例();
    下回る.パターン一覧[0].小節数 = 0;
    assert!(下回る.検証する().is_err());

    let mut 上回る = crate::common::楽曲の例();
    上回る.パターン一覧[0].小節数 = editor_server::パターンの小節数の上限 + 1;
    assert!(上回る.検証する().is_err());
}

#[test]
fn 格子の行の長さは小節数から決まるステップ数に従う() {
    let mut 対象 = crate::common::楽曲の例();
    対象.パターン一覧[0].小節数 = 3;
    // 格子は既定の小節数(2、32ステップ)のまま変えないため、小節数3が要求する48ステップと食い違う。
    assert!(対象.検証する().is_err());
}

#[test]
fn 小節数3のパターンは直列化を往復できる() {
    let mut 対象 = crate::common::楽曲の例();
    対象.パターン一覧[0].小節数 = 3;
    let ステップ数 = 3 * editor_server::小節あたりのステップ数;
    for 格子 in &mut 対象.パターン一覧[0].格子 {
        for 行 in &mut 格子.行一覧 {
            *行 = vec![0; ステップ数];
        }
    }
    対象.検証する().unwrap();

    let json = serde_json::to_string(&対象.パターン一覧[0]).unwrap();
    let 復元: editor_server::パターン = serde_json::from_str(&json).unwrap();
    assert_eq!(復元, 対象.パターン一覧[0]);
}
